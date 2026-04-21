/**
 * Application status constants
 */
export const STATUS = {
  // User status
  USER: {
    ACTIVE: 1,
    REGISTERED: 2,
    LOCKED: 3,
  },
  
  // Project status
  PROJECT: {
    ACTIVE: 1,
    CLOSED: 5,
    ARCHIVED: 9,
  },
} as const;

/**
 * Issue relation types
 */
export const ISSUE_RELATION_TYPES = {
  RELATES: "relates",
  DUPLICATES: "duplicates",
  DUPLICATED: "duplicated",
  BLOCKS: "blocks",
  BLOCKED: "blocked",
  PRECEDES: "precedes",
  FOLLOWS: "follows",
  COPIED_TO: "copied_to",
  COPIED_FROM: "copied_from",
} as const;

/**
 * Default language options
 */
export const LANGUAGES = {
  EN: "en",
  ZH: "zh",
} as const;

/**
 * Pagination defaults
 */
export const PAGINATION = {
  DEFAULT_PAGE: 1,
  DEFAULT_PER_PAGE: 25,
  MAX_PER_PAGE: 100,
} as const;
