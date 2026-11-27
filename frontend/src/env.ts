import { defineEnvVars } from '@sveltejs/kit/env';

export const variables = defineEnvVars({
	PUBLIC_TEST_FLAG: {
		public: true,
		description: 'Set to "Y" to return mock data instead of calling the Rust backend API (use "N" for real backend)'
	},
	PUBLIC_API_BASE_URL: {
		public: true,
		description: 'Base URL of the Rust backend API'
	}
});
