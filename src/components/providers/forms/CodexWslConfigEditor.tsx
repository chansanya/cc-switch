import { useTranslation } from "react-i18next";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";

interface CodexWslConfigEditorProps {
  value: string;
  error?: string;
  onChange: (value: string) => void;
}

export function CodexWslConfigEditor({
  value,
  error,
  onChange,
}: CodexWslConfigEditorProps) {
  const { t } = useTranslation();

  return (
    <section className="space-y-2 rounded-lg border border-border-default bg-muted/20 p-4">
      <div className="space-y-1">
        <Label htmlFor="codexWslConfig">{t("codexConfig.wslConfigToml")}</Label>
        <p className="text-xs text-muted-foreground">
          {t("codexConfig.wslConfigTomlDescription")}
        </p>
      </div>
      <Textarea
        id="codexWslConfig"
        value={value}
        onChange={(event) => onChange(event.target.value)}
        placeholder={t("codexConfig.wslConfigTomlPlaceholder")}
        className="min-h-[220px] font-mono text-xs"
      />
      {error ? <p className="text-xs text-destructive">{error}</p> : null}
    </section>
  );
}
