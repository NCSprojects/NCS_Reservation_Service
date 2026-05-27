# Reservation Service

예약 생성 및 관리를 담당하는 서비스.

## 기술 스택

| 항목 | 내용 |
|------|------|
| Language | Rust (Actix-web) |
| DB | MySQL (SQLx) |
| 통신 | REST API (인바운드), gRPC Server (타 서비스 요청 수신) |
| 서비스 등록 | Netflix Eureka Client |

## 주요 기능

### REST API (`/reservation`)
- `GET /reservation` — 오늘 예약 목록 조회
- `GET /reservation/:id` — 특정 예약 조회
- `GET /reservation/user` — 유저별 예약 목록 조회
- `POST /reservation/create` — 예약 생성
- `POST /reservation/create/manual/:user_id` — 수동 예약 생성 (관리자)
- `POST /reservation/count` — 예약 인원 수 업데이트
- `POST /reservation/use` — 예약 사용 처리
- `POST /reservation/cancellation` — 예약 취소

### gRPC Server
- Auth 서비스로부터 예약 사전 생성 요청 수신
- Notification 서비스로부터 예약 정보 조회 요청 수신
