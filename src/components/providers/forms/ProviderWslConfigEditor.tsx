import { useTranslation } from "react-i18next";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Textarea } from "@/components/ui/textarea";

interface ProviderWslConfigEditorProps {
  enabled: boolean;
  pathConfigured: boolean;
  format: "json" | "toml";
  value: string;
  error?: string;
  onEnabledChange: (enabled: boolean) => void;
  onChange: (value: string) => void;
}

export function ProviderWslConfigEditor({
  enabled,
  pathConfigured,
  format,
  value,
  error,
  onEnabledChange,
  onChange,
}: ProviderWslConfigEditorProps) {
  const { t } = useTranslation();
  const label = format === "toml" ? "config.toml" : "JSON";

  return (
    <section className="space-y-3 rounded-lg border border-border-default bg-muted/20 p-4">
      <div className="flex items-start justify-between gap-4">
        <div className="space-y-1">
          <Label>{t("provider.wslConfigToggle")}</Label>
          <p className="text-xs text-muted-foreground">
            {pathConfigured
              ? t("provider.wslConfigToggleDescription")
              : t("provider.wslConfigPathRequired")}
          </p>
        </div>
        <Switch
          checked={enabled}
          disabled={!pathConfigured && !enabled}
          onCheckedChange={onEnabledChange}
        />
      </div>

      {enabled ? (
        <div className="space-y-2 border-t border-border/50 pt-3">
          <Label htmlFor="providerWslConfig">
            {t("provider.wslConfigEditor", { format: label })}
          </Label>
          <Textarea
            id="providerWslConfig"
            value={value}
            onChange={(event) => onChange(event.target.value)}
            placeholder={t("provider.wslConfigPlaceholder", { format: label })}
            className="min-h-[220px] font-mono text-xs"
          />
          {!pathConfigured ? (
            <p className="text-xs text-destructive">
              {t("provider.wslConfigPathRequired")}
            </p>
          ) : null}
          {error ? <p className="text-xs text-destructive">{error}</p> : null}
        </div>
      ) : null}
    </section>
  );
}
