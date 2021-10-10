export interface TISConfig {
	lr2DB?: {
		scorePath?: string;
		chartPath?: string;
	};
	beatorajaDB?: {
		scorePath?: string;
		chartPath?: string;
	};
	uscDB?: {
		dbPath?: string;
	};
	authToken: string | null;
	warning: string;
}
