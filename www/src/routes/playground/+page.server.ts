import type { Channel } from '$lib/channels';
import type { Version } from '$lib/versions';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url, platform }) => {
    const shareId = url.searchParams.get('share');
    if (shareId === null) return;
    const share = await platform?.env?.SHARES.get(shareId);
    if (share) {
        let obj = await JSON.parse(share);
        if (!obj.code || !obj.version || !obj.channel) {
            throw new Error("Invalid share data");
        }
        return obj as Share;
    } else {
        return { message: "Share does not exist" };
    }
};

type Share = {
    code: string;
    version: Version;
    channel: Channel;
}

