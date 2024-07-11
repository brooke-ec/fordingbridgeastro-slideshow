export type Json = string | number | boolean | null | { [key: string]: Json | undefined } | Json[];

export type Database = {
	public: {
		Tables: {
			event: {
				Row: {
					cover_image: string | null;
					description: string | null;
					end_time: string | null;
					id: number;
					location: string | null;
					name: string;
					start_time: string;
				};
				Insert: {
					cover_image?: string | null;
					description?: string | null;
					end_time?: string | null;
					id?: number;
					location?: string | null;
					name: string;
					start_time: string;
				};
				Update: {
					cover_image?: string | null;
					description?: string | null;
					end_time?: string | null;
					id?: number;
					location?: string | null;
					name?: string;
					start_time?: string;
				};
				Relationships: [
					{
						foreignKeyName: "event_cover_image_fkey";
						columns: ["cover_image"];
						isOneToOne: false;
						referencedRelation: "resource";
						referencedColumns: ["filename"];
					},
				];
			};
			gallery: {
				Row: {
					author: number;
					caption: string;
					date: string;
					description: string | null;
					maxres: string;
					thumbnail: string;
					uuid: string;
				};
				Insert: {
					author: number;
					caption: string;
					date?: string;
					description?: string | null;
					maxres: string;
					thumbnail: string;
					uuid: string;
				};
				Update: {
					author?: number;
					caption?: string;
					date?: string;
					description?: string | null;
					maxres?: string;
					thumbnail?: string;
					uuid?: string;
				};
				Relationships: [
					{
						foreignKeyName: "gallery_author_fkey";
						columns: ["author"];
						isOneToOne: false;
						referencedRelation: "member";
						referencedColumns: ["id"];
					},
					{
						foreignKeyName: "gallery_maxres_fkey";
						columns: ["maxres"];
						isOneToOne: false;
						referencedRelation: "resource";
						referencedColumns: ["filename"];
					},
					{
						foreignKeyName: "gallery_thumbnail_fkey";
						columns: ["thumbnail"];
						isOneToOne: false;
						referencedRelation: "resource";
						referencedColumns: ["filename"];
					},
				];
			};
			keyvalue: {
				Row: {
					key: string;
					value: string;
				};
				Insert: {
					key: string;
					value: string;
				};
				Update: {
					key?: string;
					value?: string;
				};
				Relationships: [];
			};
			member: {
				Row: {
					avatar: string | null;
					id: number;
					name: string;
					webadmin: boolean;
				};
				Insert: {
					avatar?: string | null;
					id?: number;
					name: string;
					webadmin: boolean;
				};
				Update: {
					avatar?: string | null;
					id?: number;
					name?: string;
					webadmin?: boolean;
				};
				Relationships: [
					{
						foreignKeyName: "member_avatar_fkey";
						columns: ["avatar"];
						isOneToOne: false;
						referencedRelation: "resource";
						referencedColumns: ["filename"];
					},
				];
			};
			resource: {
				Row: {
					filename: string;
				};
				Insert: {
					filename: string;
				};
				Update: {
					filename?: string;
				};
				Relationships: [];
			};
		};
		Views: {
			[_ in never]: never;
		};
		Functions: {
			[_ in never]: never;
		};
		Enums: {
			[_ in never]: never;
		};
		CompositeTypes: {
			[_ in never]: never;
		};
	};
};

type PublicSchema = Database[Extract<keyof Database, "public">];

export type Tables<
	PublicTableNameOrOptions extends
		| keyof (PublicSchema["Tables"] & PublicSchema["Views"])
		| { schema: keyof Database },
	TableName extends PublicTableNameOrOptions extends { schema: keyof Database }
		? keyof (Database[PublicTableNameOrOptions["schema"]]["Tables"] &
				Database[PublicTableNameOrOptions["schema"]]["Views"])
		: never = never,
> = PublicTableNameOrOptions extends { schema: keyof Database }
	? (Database[PublicTableNameOrOptions["schema"]]["Tables"] &
			Database[PublicTableNameOrOptions["schema"]]["Views"])[TableName] extends {
			Row: infer R;
	  }
		? R
		: never
	: PublicTableNameOrOptions extends keyof (PublicSchema["Tables"] & PublicSchema["Views"])
	? (PublicSchema["Tables"] & PublicSchema["Views"])[PublicTableNameOrOptions] extends {
			Row: infer R;
	  }
		? R
		: never
	: never;

export type TablesInsert<
	PublicTableNameOrOptions extends keyof PublicSchema["Tables"] | { schema: keyof Database },
	TableName extends PublicTableNameOrOptions extends { schema: keyof Database }
		? keyof Database[PublicTableNameOrOptions["schema"]]["Tables"]
		: never = never,
> = PublicTableNameOrOptions extends { schema: keyof Database }
	? Database[PublicTableNameOrOptions["schema"]]["Tables"][TableName] extends {
			Insert: infer I;
	  }
		? I
		: never
	: PublicTableNameOrOptions extends keyof PublicSchema["Tables"]
	? PublicSchema["Tables"][PublicTableNameOrOptions] extends {
			Insert: infer I;
	  }
		? I
		: never
	: never;

export type TablesUpdate<
	PublicTableNameOrOptions extends keyof PublicSchema["Tables"] | { schema: keyof Database },
	TableName extends PublicTableNameOrOptions extends { schema: keyof Database }
		? keyof Database[PublicTableNameOrOptions["schema"]]["Tables"]
		: never = never,
> = PublicTableNameOrOptions extends { schema: keyof Database }
	? Database[PublicTableNameOrOptions["schema"]]["Tables"][TableName] extends {
			Update: infer U;
	  }
		? U
		: never
	: PublicTableNameOrOptions extends keyof PublicSchema["Tables"]
	? PublicSchema["Tables"][PublicTableNameOrOptions] extends {
			Update: infer U;
	  }
		? U
		: never
	: never;

export type Enums<
	PublicEnumNameOrOptions extends keyof PublicSchema["Enums"] | { schema: keyof Database },
	EnumName extends PublicEnumNameOrOptions extends { schema: keyof Database }
		? keyof Database[PublicEnumNameOrOptions["schema"]]["Enums"]
		: never = never,
> = PublicEnumNameOrOptions extends { schema: keyof Database }
	? Database[PublicEnumNameOrOptions["schema"]]["Enums"][EnumName]
	: PublicEnumNameOrOptions extends keyof PublicSchema["Enums"]
	? PublicSchema["Enums"][PublicEnumNameOrOptions]
	: never;
