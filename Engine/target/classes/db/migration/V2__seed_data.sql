
INSERT INTO scenarios (name, type, description, parameters, is_active, created_at) VALUES
                                                                                       (
                                                                                           'DNS Outage - India Region',
                                                                                           'DNS_FAILURE',
                                                                                           'Simulates complete DNS resolution failure for India region',
                                                                                           '{"failureRate": 0.3, "latencyMs": 2000, "affectedDomains": ["api.example.com", "cdn.example.com"]}',
                                                                                           true,
                                                                                           CURRENT_TIMESTAMP
                                                                                       ),
                                                                                       (
                                                                                           'CDN Edge Failure - US East',
                                                                                           'CDN_OUTAGE',
                                                                                           'Simulates CDN edge node failure in US East region',
                                                                                           '{"edgeNodes": ["us-east-1", "us-east-2"], "cacheInvalidation": true, "failureDuration": 120}',
                                                                                           true,
                                                                                           CURRENT_TIMESTAMP
                                                                                       ),
                                                                                       (
                                                                                           'Database Slow Queries',
                                                                                           'DB_LATENCY',
                                                                                           'Simulates database connection pool exhaustion and slow queries',
                                                                                           '{"slowQueryThreshold": 5000, "poolExhaustion": true, "replicaLag": 3000}',
                                                                                           true,
                                                                                           CURRENT_TIMESTAMP
                                                                                       ),
                                                                                       (
                                                                                           'Complete Region Outage - Singapore',
                                                                                           'REGION_OUTAGE',
                                                                                           'Simulates complete region failure with cascading effects',
                                                                                           '{"affectedServices": ["DNS", "CDN", "API", "DB"], "cascadeDuration": 180}',
                                                                                           true,
                                                                                           CURRENT_TIMESTAMP
                                                                                       );


DO $$
DECLARE
exp_id UUID;
    start_time TIMESTAMP;
    end_time TIMESTAMP;
BEGIN
    -- Experiment 1: COMPLETED DNS Failure
    start_time := CURRENT_TIMESTAMP - INTERVAL '2 hours';
    end_time := CURRENT_TIMESTAMP - INTERVAL '1 hour';
INSERT INTO experiments (id, name, type, region, severity, status, duration, configuration, started_at, completed_at, created_at, updated_at)
VALUES (gen_random_uuid(), 'DNS Outage Test - India', 'DNS_FAILURE', 'AP_SOUTH', 'HIGH', 'COMPLETED', 60, '{"failureRate": 0.3, "latencyMs": 2000}', start_time, end_time, start_time, end_time);

-- Experiment 2: RUNNING CDN Outage
start_time := CURRENT_TIMESTAMP - INTERVAL '15 minutes';
INSERT INTO experiments (id, name, type, region, severity, status, duration, configuration, started_at, created_at, updated_at)
VALUES (gen_random_uuid(), 'CDN Edge Failure - US East', 'CDN_OUTAGE', 'US_EAST', 'MEDIUM', 'RUNNING', 120, '{"edgeNodes": ["us-east-1"], "cacheInvalidation": true}', start_time, start_time, start_time);

-- Experiment 3: COMPLETED Database Latency
start_time := CURRENT_TIMESTAMP - INTERVAL '3 hours';
    end_time := CURRENT_TIMESTAMP - INTERVAL '2 hours';
INSERT INTO experiments (id, name, type, region, severity, status, duration, configuration, started_at, completed_at, created_at, updated_at)
VALUES (gen_random_uuid(), 'Database Slow Query Test', 'DB_LATENCY', 'EU_WEST', 'MEDIUM', 'COMPLETED', 90, '{"slowQueryThreshold": 5000, "poolExhaustion": true}', start_time, end_time, start_time, end_time);

-- Experiment 4: FAILED Region Outage
start_time := CURRENT_TIMESTAMP - INTERVAL '1 hour';
    end_time := CURRENT_TIMESTAMP - INTERVAL '45 minutes';
INSERT INTO experiments (id, name, type, region, severity, status, duration, configuration, started_at, completed_at, created_at, updated_at)
VALUES (gen_random_uuid(), 'Singapore Region Outage Test', 'REGION_OUTAGE', 'AP_SOUTHEAST', 'HIGH', 'FAILED', 30, '{"affectedServices": ["DNS", "CDN", "API"]}', start_time, end_time, start_time, end_time);

-- Experiment 5: PENDING (Waiting to start)
INSERT INTO experiments (id, name, type, region, severity, status, duration, configuration, created_at, updated_at)
VALUES (gen_random_uuid(), 'DNS Cache Poisoning Test', 'DNS_FAILURE', 'US_WEST', 'LOW', 'PENDING', 45, '{"cachePoisoning": true, "ttlModification": 0}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Experiment 6: COMPLETED with cascading failure
start_time := CURRENT_TIMESTAMP - INTERVAL '4 hours';
    end_time := CURRENT_TIMESTAMP - INTERVAL '3.5 hours';
INSERT INTO experiments (id, name, type, region, severity, status, duration, configuration, started_at, completed_at, created_at, updated_at)
VALUES (gen_random_uuid(), 'Cascading DNS to API Failure', 'DNS_FAILURE', 'AP_SOUTH', 'HIGH', 'COMPLETED', 180, '{"failureRate": 0.5, "cascadeTo": ["API", "CDN"]}', start_time, end_time, start_time, end_time);

-- Experiment 7: RUNNING Database + CDN combo
start_time := CURRENT_TIMESTAMP - INTERVAL '5 minutes';
INSERT INTO experiments (id, name, type, region, severity, status, duration, configuration, started_at, created_at, updated_at)
VALUES (gen_random_uuid(), 'Database + CDN Combined Failure', 'DB_LATENCY', 'US_EAST', 'HIGH', 'RUNNING', 60, '{"dbLatency": 3000, "cdnLatency": 2000, "combined": true}', start_time, start_time, start_time);

-- Experiment 8: CANCELLED
start_time := CURRENT_TIMESTAMP - INTERVAL '30 minutes';
    end_time := CURRENT_TIMESTAMP - INTERVAL '28 minutes';
INSERT INTO experiments (id, name, type, region, severity, status, duration, configuration, started_at, completed_at, created_at, updated_at)
VALUES (gen_random_uuid(), 'Cancelled CDN Test', 'CDN_OUTAGE', 'EU_WEST', 'LOW', 'CANCELLED', 30, '{"edgeNodes": ["eu-west-1"]}', start_time, end_time, start_time, end_time);

-- Experiment 9: COMPLETED Regional
start_time := CURRENT_TIMESTAMP - INTERVAL '6 hours';
    end_time := CURRENT_TIMESTAMP - INTERVAL '5.5 hours';
INSERT INTO experiments (id, name, type, region, severity, status, duration, configuration, started_at, completed_at, created_at, updated_at)
VALUES (gen_random_uuid(), 'EU West Complete Outage', 'REGION_OUTAGE', 'EU_WEST', 'HIGH', 'COMPLETED', 180, '{"totalRegionOutage": true, "failoverEnabled": false}', start_time, end_time, start_time, end_time);

-- Experiment 10: RUNNING DNS test
start_time := CURRENT_TIMESTAMP - INTERVAL '10 minutes';
INSERT INTO experiments (id, name, type, region, severity, status, duration, configuration, started_at, created_at, updated_at)
VALUES (gen_random_uuid(), 'DNS Slow Resolution Test', 'DNS_FAILURE', 'AP_SOUTH', 'MEDIUM', 'RUNNING', 300, '{"resolutionDelay": 5000, "timeoutRate": 0.15}', start_time, start_time, start_time);
END $$;

-- ============================================
-- 3. SEED: METRICS DATA (Time-series Simulation)
-- ============================================

-- Generate 100 metrics points for each experiment
DO $$
DECLARE
exp_record RECORD;
    base_time TIMESTAMP;
    metric_count INTEGER;
    dns_latency DOUBLE PRECISION;
    cdn_latency DOUBLE PRECISION;
    db_latency DOUBLE PRECISION;
    health_score DOUBLE PRECISION;
    packet_loss DOUBLE PRECISION;
    error_rate DOUBLE PRECISION;
    time_offset INTEGER;
BEGIN
FOR exp_record IN SELECT id, started_at, type FROM experiments WHERE started_at IS NOT NULL LOOP
        -- Generate 50-100 metrics per experiment
        FOR metric_count IN 1..FLOOR(RANDOM() * 50 + 50) LOOP
            time_offset := metric_count * 2; -- Every 2 seconds
base_time := exp_record.started_at + (time_offset || ' seconds')::INTERVAL;

            -- Generate realistic metric values based on experiment type
CASE exp_record.type
                WHEN 'DNS_FAILURE' THEN
                    dns_latency := 50 + (RANDOM() * 2000);
                    cdn_latency := 100 + (RANDOM() * 500);
                    db_latency := 20 + (RANDOM() * 200);
                    packet_loss := RANDOM() * 0.3;
                    error_rate := RANDOM() * 0.2;
                    health_score := 100 - (dns_latency / 2000 * 50) - (packet_loss * 200);

WHEN 'CDN_OUTAGE' THEN
                    dns_latency := 20 + (RANDOM() * 100);
                    cdn_latency := 500 + (RANDOM() * 3000);
                    db_latency := 20 + (RANDOM() * 200);
                    packet_loss := RANDOM() * 0.1;
                    error_rate := RANDOM() * 0.3;
                    health_score := 100 - (cdn_latency / 3000 * 70) - (error_rate * 200);

WHEN 'DB_LATENCY' THEN
                    dns_latency := 20 + (RANDOM() * 100);
                    cdn_latency := 100 + (RANDOM() * 200);
                    db_latency := 500 + (RANDOM() * 4000);
                    packet_loss := RANDOM() * 0.05;
                    error_rate := RANDOM() * 0.15;
                    health_score := 100 - (db_latency / 4000 * 80);

WHEN 'REGION_OUTAGE' THEN
                    dns_latency := 100 + (RANDOM() * 1000);
                    cdn_latency := 200 + (RANDOM() * 2000);
                    db_latency := 100 + (RANDOM() * 1000);
                    packet_loss := RANDOM() * 0.5;
                    error_rate := RANDOM() * 0.5;
                    health_score := 100 - (packet_loss * 150) - (error_rate * 100);

ELSE
                    dns_latency := 20 + (RANDOM() * 100);
                    cdn_latency := 50 + (RANDOM() * 200);
                    db_latency := 10 + (RANDOM() * 100);
                    packet_loss := RANDOM() * 0.05;
                    error_rate := RANDOM() * 0.05;
                    health_score := 90 + (RANDOM() * 10);
END CASE;

            -- Ensure values stay within reasonable bounds
            health_score := GREATEST(0, LEAST(100, health_score));
            error_rate := GREATEST(0, LEAST(1, error_rate));

INSERT INTO metrics (
    experiment_id,
    timestamp,
    dns_resolution_time_ms,
    dns_success_rate,
    cdn_latency_ms,
    cdn_availability,
    db_query_time_ms,
    db_connection_pool_usage,
    packet_loss_percent,
    network_latency_ms,
    error_rate,
    region_health_score,
    additional_metrics
) VALUES (
             exp_record.id,
             base_time,
             GREATEST(10, dns_latency),
             GREATEST(0, 100 - (error_rate * 100)),
             GREATEST(10, cdn_latency),
             GREATEST(0, 100 - (packet_loss * 100)),
             GREATEST(10, db_latency),
             20 + (RANDOM() * 80),
             packet_loss * 100,
             dns_latency * 0.5 + cdn_latency * 0.3 + db_latency * 0.2,
             error_rate,
             health_score,
             jsonb_build_object(
                     'dnsHealth', GREATEST(0, 100 - (dns_latency / 2000 * 50)),
                     'cdnHealth', GREATEST(0, 100 - (cdn_latency / 3000 * 70)),
                     'dbHealth', GREATEST(0, 100 - (db_latency / 4000 * 80)),
                     'networkHealth', GREATEST(0, 100 - (packet_loss * 200))
             )
         );
END LOOP;
END LOOP;
END $$;

INSERT INTO alerts (experiment_id, severity, message, timestamp, acknowledged, resolved_at)
SELECT
    e.id,
    CASE
        WHEN m.error_rate > 0.3 THEN 'CRITICAL'
        WHEN m.error_rate > 0.15 THEN 'WARNING'
        ELSE 'INFO'
        END as severity,
    CASE
        WHEN m.error_rate > 0.3 THEN 'Critical error rate detected: ' || ROUND(m.error_rate * 100) || '%'
        WHEN m.packet_loss_percent > 20 THEN 'High packet loss: ' || ROUND(m.packet_loss_percent) || '%'
        WHEN m.region_health_score < 50 THEN 'Region health critical: ' || ROUND(m.region_health_score) || '%'
        ELSE 'Degraded performance detected'
        END as message,
    m.timestamp,
    CASE WHEN RANDOM() > 0.7 THEN true ELSE false END,
    CASE WHEN RANDOM() > 0.7 THEN m.timestamp + INTERVAL '5 minutes' ELSE NULL END
FROM metrics m
    JOIN experiments e ON e.id = m.experiment_id
WHERE m.error_rate > 0.1 OR m.packet_loss_percent > 10 OR m.region_health_score < 70
    LIMIT 30;


INSERT INTO audit_logs (user_id, action, details, timestamp, ip_address)
SELECT
    'system_user_' || (RANDOM() * 100)::INT,
    CASE floor(random() * 5)::INT
        WHEN 0 THEN 'EXPERIMENT_START'
        WHEN 1 THEN 'EXPERIMENT_STOP'
        WHEN 2 THEN 'EXPERIMENT_VIEW'
        WHEN 3 THEN 'ALERT_ACKNOWLEDGE'
        ELSE 'SYSTEM_ACTION'
END,
    jsonb_build_object(
        'experiment_id', e.id,
        'action_time', (CURRENT_TIMESTAMP - (RANDOM() * INTERVAL '1 day')),
        'details', 'User action performed'
    ),
    CURRENT_TIMESTAMP - (RANDOM() * INTERVAL '1 day'),
    '192.168.' || (RANDOM() * 255)::INT || '.' || (RANDOM() * 255)::INT
FROM experiments e
WHERE RANDOM() > 0.5
LIMIT 20;


DO $$
BEGIN
    RAISE NOTICE 'Data seeding complete.';
    RAISE NOTICE 'Scenarios: %', (SELECT COUNT(*) FROM scenarios);
    RAISE NOTICE 'Experiments: %', (SELECT COUNT(*) FROM experiments);
    RAISE NOTICE 'Metrics: %', (SELECT COUNT(*) FROM metrics);
    RAISE NOTICE '!!!!!Alerts: %', (SELECT COUNT(*) FROM alerts);
    RAISE NOTICE 'Audit Logs: %', (SELECT COUNT(*) FROM audit_logs);
END $$;