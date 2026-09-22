-- Add migration script here
CREATE TABLE stock_reservations(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    idempotency_key VARCHAR(255) NOT NULL UNIQUE,
    product_id UUID NOT NULL,   
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_stock_reservations_product_id
    ON stock_reservations(product_id);