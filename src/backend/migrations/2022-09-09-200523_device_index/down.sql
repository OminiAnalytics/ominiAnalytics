-- This file should undo anything in `up.sql`

DROP TABLE IF EXISTS omini_device_info_index;

DROP INDEX IF EXISTS device_idx_ua;

DROP INDEX IF EXISTS device_idx_os;

DROP INDEX IF EXISTS device_idx_browser;

DROP INDEX IF EXISTS device_idx_country;

DROP INDEX IF EXISTS device_idx_cpu;

DROP INDEX IF EXISTS device_idx_gpu;

DROP INDEX IF EXISTS device_idx_memory;

DROP INDEX IF EXISTS device_idx_screen;