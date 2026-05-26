#include <Arduino.h>
#include <ArduinoLog.h>
#include <FastLED.h>
#include <WsClient.h>

#include "logging.h"

CRGB leds[LED_COUNT];
WsClient wsClient;

void safeBoot();
void setupLEDS();
void setupWifi();

void setup()
{
	Serial.begin(115200);
	while (!Serial && !Serial.available()) { }

	setupLogging();
	safeBoot();
	setupLEDS();
	setupWifi();
}

void loop()
{
	while (wsClient.getStatus() == WsClient::Status::DISCONNECTED) {
		delay(1000);
		wsClient.connect(WS_HOST, WS_PORT, WS_PATH);
	}

	bool receivedPayload = wsClient.poll();
	if (receivedPayload) {
		Payload payload = wsClient.getLatestPayload();
		Log.infoln("Received payload (opcode=%d, length=%d)", payload.opcode, payload.length);

		if (payload.opcode != Opcode::BINARY) {
			Log.warningln("Ignoring non-binary websocket frame (opcode=%d)", payload.opcode);
			return;
		}

		for (size_t i = 0; i < LED_COUNT; ++i) {
			size_t redChannelIndex = i * 3;
			leds[i] = CRGB(
				payload.data[redChannelIndex],
				payload.data[redChannelIndex + 1],
				payload.data[redChannelIndex + 2]);
		}

		FastLED.show();
	}
}

void safeBoot()
{
	for (uint8_t t = 4; t > 0; t--) {
		Log.infoln("[SETUP] Boot Wait (%d)", t);
		delay(1000);
	}
}

void setupLEDS()
{
	FastLED.addLeds<WS2811, 4, RGB>(leds, LED_COUNT);
	FastLED.setBrightness(100);
	FastLED.clear(true);

	for (size_t i = 0; i < 5; ++i) {
		Log.infoln("[LEDS] Blink %d", i);

		for (size_t ledIndex = 0; ledIndex < LED_COUNT; ++ledIndex) {
			leds[ledIndex] = CRGB(255, 0, 0);
		}
		FastLED.show();
		delay(50);

		for (size_t ledIndex = 0; ledIndex < LED_COUNT; ++ledIndex) {
			leds[ledIndex] = CRGB(0, 0, 0);
		}
		FastLED.show();
		delay(50);
	}
}

void setupWifi()
{
	Log.infoln("[WiFi] Setting up wifi...");

	// check for the WiFi module:
	if (WiFi.status() == WL_NO_MODULE) {
		Log.errorln("[WiFi] Communication with WiFi module failed!");
		while (true) { }
	}

	String fwVersion = WiFi.firmwareVersion();
	Log.infoln("[WiFi] Firmware version: %s", fwVersion.c_str());

	wl_status_t wifiStatus = wl_status_t::WL_IDLE_STATUS;
	while (wifiStatus != WL_CONNECTED) {
		Log.infoln("[WiFi] Attempting to connect to '%s'", WIFI_SSID);

		wifiStatus = (wl_status_t)WiFi.begin(WIFI_SSID, WIFI_PASS);

		delay(500);
	}

	IPAddress ip = WiFi.localIP();
	Log.infoln("[WiFi] Connected! (ip='%p')", ip);
}
