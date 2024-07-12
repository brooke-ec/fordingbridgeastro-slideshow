export interface GalleryEntry {
	maxres: string;
	caption: string;
	member: {
		name: string;
		avatar: string | null;
	};
}
