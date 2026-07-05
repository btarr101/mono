# Modeling

```mermaid
---
config:
    layout: elk
---
erDiagram
	person {
		uuid uuid PK
		text username
		created_at timestamp
		updated_at timestamp
	}

	card {
		uuid oracle_id PK
		text name
		text image_uri
		text legality
	}

	card_rating {
		uuid uuid PK
		uuid card_oracle_id FK
		uuid rater_person_uuid FK
		numeric points
		text reason
	}

	card_rating_review {
		uuid uuid PK
		uuid reviewer_person_uuid FK
		uuid reviewed_card_rating_uuid FK
		bool liked
	}

	card ||--o{ card_rating : has
	person ||--o{ card_rating : creates
	person ||--o{ card_rating_review : creates
	card_rating ||--o{ card_rating_review : reviews
```

Additional constraints from the current schema:

- `card_rating` is unique on `(card_oracle_id, rater_person_uuid)`.
- `card_rating_review` is unique on `(reviewer_person_uuid, reviewed_card_rating_uuid)`.
