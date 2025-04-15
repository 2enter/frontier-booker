import Image from './ui/texts/2enter.webp?enhanced'

export const imageModules = import.meta.glob(
	'./**/*.{webp,png}',
	{
		eager: true,
		query: {
			enhanced: true
		}
	}
) as Record<string, { default: typeof Image }>


export function getImageSrc(src: string) {
	return imageModules[`.${src}`].default;
}
