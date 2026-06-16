import { createTheme } from '@mantine/core'

export const theme = createTheme({
  primaryColor: 'grape',

  colors: {
    grape: [
      '#f7f3ff',
      '#ede4ff',
      '#dcc8ff',
      '#c7a4ff',
      '#af7bff',
      '#9855ff',
      '#8b3dff',
      '#7c3aed', // primary
      '#6929c4',
      '#551f99',
    ],
  },

  black: '#111111',
  white: '#ffffff',

  primaryShade: 7,

  fontFamily: 'Inter, ui-sans-serif, system-ui, sans-serif',

  headings: {
    fontFamily: 'Inter, ui-sans-serif, system-ui, sans-serif',
    fontWeight: '800',
  },

  components: {
    NavLink: {
      defaultProps: {
        variant: 'filled',
      },
    },

    // Paper: {
    //   defaultProps: {
    //     shadow: undefined,
    //     radius: 0,
    //     withBorder: true,
    //   },
    // },

    // Card: {
    //   styles: {
    //     root: {
    //       border: '3px solid #111',
    //       // boxShadow: '6px 6px 0px #111',
    //     },
    //   },
    // },

    // Button: {
    //   styles: {
    //     root: {
    //       border: '3px solid #111',
    //       boxShadow: '4px 4px 0px #111',
    //       fontWeight: 700,
    //     },
    //   },
    // },

    // TextInput: {
    //   styles: {
    //     input: {
    //       border: '3px solid #111',
    //     },
    //   },
    // },
  },
})
