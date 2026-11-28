# Backend API Endpoints Specification

This document defines the HTTP + JSON contract expected by the SvelteKit frontend (`frontend/src/lib/api/exams.ts` and `frontend/src/lib/types/exam.ts`) for the Rust + PostgreSQL backend.

## Base Configuration

- **Base URL:** Configured via `PUBLIC_API_BASE_URL` in `.env` (default: `http://localhost:8080/api`). Endpoint paths below are relative to this base URL (e.g. `/exams`, `/exams/:id`).
- **Mock Toggle:** Set `PUBLIC_TEST_FLAG=N` in `.env` to switch the frontend from mock data to these real backend endpoints (`Y` for mock mode).
- **CORS:** Configured via `CORS_ORIGIN` in `.env` (comma-separated origins, default `http://localhost:5173`; e.g. `https://dump.ryu.mooo.com` in production), allowing methods `GET, POST, PUT, DELETE, OPTIONS` and headers `Content-Type, X-Client-Id`.

## Common Request Headers

| Header | Required | Description |
| :--- | :--- | :--- |
| `Content-Type` | Yes (on `POST`) | `application/json` |
| `X-Client-Id` | Recommended | Client identifier (`3–64` chars: `[a-zA-Z0-9_-]`, UUID v4 auto-generated on first load in `localStorage` key `dump_client_id`, editable via UI) used to isolate and persist user progress per `(client_id, exam_id)`. |

---

## Endpoints

### 1. List Exams (Paginated, Searchable, Filterable)

Returns a paginated list of exam summaries with the client's current progress.

- **Method & Path:** `GET /api/exams`
- **Query Parameters:**

| Param | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `page` | integer (`>= 1`) | `1` | Page number (1-based). |
| `limit` | integer (`>= 1`) | `6` | Items per page. |
| `q` | string | `""` | Case-insensitive search across `code`, `title`, and `vendor`. |
| `status` | string | omitted (`all`) | Filter by progress: `in_progress`, `not_started`, or `completed`. |

- **Response (`200 OK` — `PaginatedExamsResponse`):**

```json
{
  "items": [
    {
      "id": "mb-330",
      "code": "MB-330",
      "title": "Microsoft Dynamics 365 Supply Chain Management",
      "vendor": "Microsoft",
      "totalQuestions": 120,
      "answeredCount": 42,
      "lastQuestionIndex": 43,
      "updatedAt": "2026-11-20"
    }
  ],
  "total": 34,
  "page": 1,
  "limit": 6,
  "totalPages": 6
}
```

---

### 2. Get Exam Overview

Returns full metadata and per-topic progress for a single exam without loading the questions array.

- **Method & Path:** `GET /api/exams/:id`
- **Path Parameters:**
  - `id`: Exam identifier (e.g. `mb-330`).
- **Response (`200 OK` — `ExamDetail`):**

```json
{
  "id": "mb-330",
  "code": "MB-330",
  "title": "Microsoft Dynamics 365 Supply Chain Management",
  "vendor": "Microsoft",
  "totalQuestions": 120,
  "answeredCount": 42,
  "lastQuestionIndex": 43,
  "updatedAt": "2026-11-20",
  "description": "Practice questions covering product information, inventory control, procurement, sales, and warehouse management in Dynamics 365 Supply Chain Management.",
  "descriptionVi": "Bộ câu hỏi luyện tập về quản lý thông tin sản phẩm, kiểm soát tồn kho, mua hàng, bán hàng và quản lý kho trong Dynamics 365 Supply Chain Management.",
  "topics": [
    {
      "id": "pim",
      "name": "Implement Product Information Management",
      "nameVi": "Triển khai quản lý thông tin sản phẩm",
      "questionCount": 30,
      "answeredCount": 30,
      "startQuestionIndex": 1,
      "continueQuestionIndex": null
    },
    {
      "id": "inv",
      "name": "Implement Inventory Management",
      "nameVi": "Triển khai quản lý tồn kho",
      "questionCount": 35,
      "answeredCount": 12,
      "startQuestionIndex": 31,
      "continueQuestionIndex": 43
    }
  ]
}
```

> **Note on `continueQuestionIndex`:** Only the single topic containing `lastQuestionIndex` should return a non-null `continueQuestionIndex` (equal to `lastQuestionIndex`); all other topics return `null`.

---

### 3. Get Single Question for Practice

Loads one question by its actual exam question number (`q`), optionally scoped to a topic (`topic`), and **records that question as `lastQuestionIndex`** for the client.

- **Method & Path:** `GET /api/exams/:id/questions`
- **Path Parameters:**
  - `id`: Exam identifier (e.g. `mb-330`).
- **Query Parameters:**

| Param | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `q` | integer | `lastQuestionIndex` or first question in scope | Actual 1-based exam question number (`question.index`, e.g. `31`–`65` for topic `inv`). |
| `topic` | string | omitted | Optional `topicId` to filter the navigator and Next/Previous scope to one topic. |
| `case_study` | string | omitted | Optional `caseStudyId` to filter the navigator and Next/Previous scope to one Case Study. |
| `filter` | string | omitted (`all`) | Optional scope filter: `all`, `wrong` (previously answered incorrectly), or `flagged` (marked for review). |

- **Side Effect:** Updates `last_question_index = question.index` in PostgreSQL for `(client_id, exam_id)`.
- **Response (`200 OK` — `ExamQuestionPayload`):**

```json
{
  "exam": {
    "id": "mb-330",
    "code": "MB-330",
    "title": "Microsoft Dynamics 365 Supply Chain Management",
    "vendor": "Microsoft",
    "totalQuestions": 459,
    "answeredCount": 42,
    "lastQuestionIndex": 43,
    "updatedAt": "2026-11-28",
    "description": "...",
    "descriptionVi": "...",
    "topics": [
      {
        "id": "pim",
        "name": "Implement Product Information Management",
        "nameVi": "Triển khai quản lý thông tin sản phẩm",
        "questionCount": 30
      }
    ]
  },
  "totalInScope": 35,
  "currentIndex": 43,
  "question": {
    "id": "mb-330-q-43",
    "topicId": "inv",
    "caseStudyId": "adventure-works",
    "index": 43,
    "questionType": "single_choice",
    "prompt": "...",
    "promptVi": "...",
    "spec": {
      "options": [
        { "id": "a", "label": "A", "text": "...", "textVi": "..." }
      ]
    },
    "answerKey": ["a"],
    "maxPoints": 1,
    "explanation": "...",
    "explanationVi": "...",
    "keywords": ["Location directives", "Restrict by unit", "Pallet"],
    "keywordsVi": ["Chỉ dẫn vị trí kho", "Giới hạn theo đơn vị", "Pallet"],
    "images": []
  },
  "caseStudy": {
    "id": "adventure-works",
    "title": "Adventure Works Cycles",
    "titleVi": "Adventure Works Cycles",
    "sections": [
      { "id": "overview", "title": "Overview", "titleVi": "Tổng quan", "content": "...", "contentVi": "..." }
    ],
    "images": []
  },
  "answer": {
    "questionId": "mb-330-q-43",
    "submittedAnswer": ["a"],
    "earnedPoints": 1,
    "maxPoints": 1,
    "isCorrect": true
  },
  "isFlagged": false,
  "navItems": [
    { "index": 31, "status": "correct", "isFlagged": false },
    { "index": 32, "status": "incorrect", "isFlagged": true },
    { "index": 43, "status": "unanswered", "isFlagged": false }
  ],
  "answeredCount": 42,
  "flaggedCount": 3
}
```

- `answer` is `null` if the client has not answered this question yet; `caseStudy` is omitted when the question is standalone.
- `questionType` is one of `"single_choice" | "multiple_choice" | "hotspot" | "drag_drop" | "simulation"`.
- `navItems[].status` is one of `"unanswered" | "correct" | "incorrect"`; `navItems[].isFlagged` is independent of `status`.
- `images` holds `{ id, url, alt? }` entries referenced inline via `[[img:<id>]]` placeholders in `prompt`/`explanation`/`caseStudy.sections`.
- Health check endpoint: `GET /health` returns `200 OK` (`{ "status": "ok" }`).

---

### 4. Submit Answer

Evaluates the submitted answer (choice array or slot/target map) with partial credit, persists the answer and `lastQuestionIndex` for `(client_id, exam_id)`, and returns the graded result.

- **Method & Path:** `POST /api/exams/:id/answers`
- **Path Parameters:**
  - `id`: Exam identifier (e.g. `mb-330`).
- **Request Body (`SubmitAnswerRequest`):**

```json
{
  "questionId": "mb-330-q-43",
  "questionIndex": 43,
  "submittedAnswer": ["a"]
}
```

*(For `hotspot` / `drag_drop`, `submittedAnswer` is an object map such as `{ "s1": "c1", "s2": "c2" }`.)*

- **Response (`200 OK` — `ExamAnswerRecord`):**

```json
{
  "questionId": "mb-330-q-43",
  "submittedAnswer": ["a"],
  "earnedPoints": 1,
  "maxPoints": 1,
  "isCorrect": true
}
```

- **`DELETE /api/exams/:id/answers/:questionId`**: Deletes the client's submitted answer for a single question (returning it to unanswered) and returns `{ "questionId": "...", "reset": true }`.
- **`DELETE /api/exams/:id/answers`**: Clears all submitted answers, `client_exam_progress`, and `client_question_flags` for `(client_id, exam_id)` and returns `{ "examId": "...", "reset": true }`.

---

### 5. Mark for Review Flags (`client_question_flags`)

Marks or unmarks a question for review (works independently both **before** and **after** answering). Requires `X-Client-Id`.

- **`PUT /api/exams/:id/flags`**: Body `{ "questionId": "...", "questionIndex": 43 }`. Upserts flag and returns `{ "questionId": "...", "isFlagged": true }`.
- **`DELETE /api/exams/:id/flags/:questionId`**: Removes the flag and returns `{ "questionId": "...", "isFlagged": false }`.

---

### 6. Community Question Notes (`question_notes`)

Shared per-question study notes visible to all users; each user can create multiple notes and edit/delete their own (`X-Client-Id`).

- **`GET /api/exams/:id/questions/:questionId/notes`**: Returns `QuestionNote[]` (`{ id, questionId, clientId, content, createdAt, updatedAt, isOwner, upvotes, downvotes, score, myVote }`) ordered by `score DESC, createdAt DESC`.
- **`POST /api/exams/:id/questions/:questionId/notes`**: Requires `X-Client-Id`. Body `{ "content": "...", "parentId"?: number }` (`1..2000` chars; `parentId` allows strictly 1-level replies and returns `400` if targeting another reply). Returns created `QuestionNote`.
- **`PUT /api/exams/:id/questions/:questionId/notes/:noteId`**: Requires `X-Client-Id` (owner only). Body `{ "content": "..." }`. Returns updated `QuestionNote`.
- **`DELETE /api/exams/:id/questions/:questionId/notes/:noteId`**: Requires `X-Client-Id` (owner only). Returns `{ "id": ..., "deleted": true }`.
- **`PUT /api/exams/:id/questions/:questionId/notes/:noteId/vote`**: Requires `X-Client-Id`. Body `{ "vote": 1 | -1 | 0 }` (`0` removes vote). Returns `{ noteId, upvotes, downvotes, score, myVote }`.

---

### 7. Register / Ensure / Rename / Sync Client ID

- **`POST /api/clients/register`**: Registers a newly generated `X-Client-Id` if not already present in `clients` (or allocates a fresh unique UUID v4 if a collision occurs). Returns `{ "clientId": "..." }`.
- **`POST /api/clients/ensure`**: Ensures an existing `X-Client-Id` from `localStorage` is recorded in `clients` without overwriting it. Returns `{ "clientId": "..." }`.
- **`PUT /api/clients/id`**: Renames the current `X-Client-Id` to `{ "newClientId": "..." }` (`3–64` chars `[a-zA-Z0-9_-]`), migrating all `client_exam_progress` and `client_answers` via `ON UPDATE CASCADE`. Returns `409 Conflict` if `newClientId` already exists.
- **`POST /api/clients/sync`**: Syncs to an existing `{ "targetClientId": "..." }` from another device. Verifies `targetClientId` exists in `clients` (returns `404 Not Found` otherwise), transfers `question_notes` and non-conflicting `question_note_votes` from the current `X-Client-Id` to `targetClientId` (if both IDs voted on the same note, `targetClientId`'s vote is preserved), deletes the old `X-Client-Id` row from `clients` (cascading to `client_exam_progress`, `client_answers`, `client_question_flags`, and any duplicate votes), and returns `{ "clientId": targetClientId }`.

---

## Error Responses

Non-2xx responses (e.g. `400 Bad Request`, `404 Not Found`, `409 Conflict`, `500 Internal Server Error`) are surfaced by `apiRequest` in `frontend/src/lib/api/client.ts` using the JSON `{ "error": "..." }` message or HTTP status.
