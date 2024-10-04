<script lang="ts">
    import { Dialog as DialogPrimitive } from "bits-ui";
    interface Props {
        [key: string]: any;
    }

    let { children, ...rest }: Props = $props();
    type $$Props = DialogPrimitive.PortalProps;
</script>

<DialogPrimitive.Portal {...rest}>
    {@render children?.()}
</DialogPrimitive.Portal>
