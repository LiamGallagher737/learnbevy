// Javascript port of https://github.com/bevyengine/disqualified

export class TypePath {
    private fullTypePath: string;
    private shortTypeName?: string;

    constructor(fullName: string) {
        this.fullTypePath = fullName;
    }

    full(): string {
        return this.fullTypePath;
    }

    short(): string {
        if (this.shortTypeName) {
            return this.shortTypeName;
        }

        let result = "";
        let index = 0;
        const endOfString = this.fullTypePath.length;

        while (index < endOfString) {
            const restOfString = this.fullTypePath.slice(index);

            const specialCharacterIndex = restOfString.search(/[\s<>()[\],;]/);
            if (specialCharacterIndex !== -1) {
                const segmentToCollapse = restOfString.slice(0, specialCharacterIndex);
                result += collapseTypeName(segmentToCollapse);

                const specialCharacter = restOfString[specialCharacterIndex];
                result += specialCharacter;

                if (
                    [">", ")", "]"].includes(specialCharacter) &&
                    restOfString.startsWith("::", specialCharacterIndex + 1)
                ) {
                    result += "::";
                    index += specialCharacterIndex + 3;
                } else {
                    index += specialCharacterIndex + 1;
                }
            } else {
                result += collapseTypeName(restOfString);
                index = endOfString;
            }
        }

        this.shortTypeName = result;
        return result;
    }
}

function collapseTypeName(input: string) {
    const segments = input.split("::");
    const last = segments.pop();
    const secondLast = segments.pop();

    if (secondLast && /^[A-Z]/.test(secondLast)) {
        return `${secondLast}::${last}`;
    } else {
        return last;
    }
}
