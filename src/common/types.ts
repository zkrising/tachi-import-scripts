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
		playtype?: "Controller" | "Keyboard";
	};
	authToken: string | null;
	warning: string;
	staging?: boolean;
}
