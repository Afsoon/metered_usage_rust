CREATE DATABASE analytics;
CREATE TABLE "analytics"."metered_usage"
(
    customer_id UUID,
    usage_type Enum16(
        'API_CALL' = 1,
        'STORAGE' = 2,
        'EXECUTION_TIME' = 3
    ),
    created_at DateTime64(3),
)
ENGINE = MergeTree
PARTITION BY toYYYYMM(created_at)
ORDER BY (customer_id, created_at)
