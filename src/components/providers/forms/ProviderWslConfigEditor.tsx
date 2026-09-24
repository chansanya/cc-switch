import { useTranslation } from "react-i18next";
import JsonEditor from "@/components/JsonEditor";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { useDarkMode } from "@/hooks/useDarkMode";

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
  const isDarkMode = useDarkMode();
  const label = format === "toml" ? "config.toml (TOML)" : "JSON";
  const editorHeight = Math.max(10, value.split("\n").length) * 20 + 20;

  return (
    <section className="space-y-4 border-t border-border/60 pt-5">
      <div className="grid grid-cols-[minmax(0,1fr)_auto] items-start gap-6">
        <div className="space-y-1">
          <Label>{t("provider.wslConfigToggle")}</Label>
          <p className="text-xs text-muted-foreground">
            {pathConfigured
              ? t("provider.wslConfigToggleDescription")
              : t("provider.wslConfigPathRequired")}
          </p>
        </div>
        <div className="flex min-w-12 justify-end pt-0.5">
          <Switch
            checked={enabled}
            disabled={!pathConfigured && !enabled}
            onCheckedChange={onEnabledChange}
          />
        </div>
      </div>

      {enabled ? (
        <div className="space-y-2">
          <Label htmlFor="providerWslConfig">
            {t("provider.wslConfigEditor", { format: label })}
          </Label>
          <JsonEditor
            id="providerWslConfig"
            ariaLabel={t("provider.wslConfigEditor", { format: label })}
            value={value}
            onChange={onChange}
            placeholder={t("provider.wslConfigPlaceholder", { format: label })}
            darkMode={isDarkMode}
            height={editorHeight}
            showValidation={format === "json"}
            language={format === "json" ? "json" : "javascript"}
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
