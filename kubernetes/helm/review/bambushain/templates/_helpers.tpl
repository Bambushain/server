{{/*
Expand the name of the chart.
*/}}
{{- define "bambushain.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create a default fully qualified app name.
We truncate at 63 chars because some Kubernetes name fields are limited to this (by the DNS naming spec).
If release name contains chart name it will be used as a full name.
*/}}
{{- define "bambushain.fullname" -}}
{{- if .Values.fullnameOverride }}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- $name := default .Chart.Name .Values.nameOverride }}
{{- if contains $name .Release.Name }}
{{- .Release.Name | trunc 63 | trimSuffix "-" }}
{{- else }}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" }}
{{- end }}
{{- end }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "bambushain.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "bambushain.labels" -}}
helm.sh/chart: {{ include "bambushain.chart" . }}
{{ include "bambushain.selectorLabels" . }}
{{- if .Chart.AppVersion }}
app.kubernetes.io/version: {{ .Chart.AppVersion | quote }}
{{- end }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
bambushain.app/branch: {{ .Values.branch | default .Release.Name | quote }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "bambushain.selectorLabels" -}}
app.kubernetes.io/name: {{ include "bambushain.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Create the name of the service account to use
*/}}
{{- define "bambushain.serviceAccountName" -}}
{{- if .Values.serviceAccount.create }}
{{- default (include "bambushain.fullname" .) .Values.serviceAccount.name }}
{{- else }}
{{- default "default" .Values.serviceAccount.name }}
{{- end }}
{{- end }}

{{/*
Get the full image name
*/}}
{{- define "bambushain.imageFullName" -}}
{{- $registry := .global.registry -}}
{{- $repository := .repository -}}
{{- $tag := .tag | default .global.tag -}}
{{- printf "%s/%s:%s" $registry $repository $tag -}}
{{- end }}

{{/*
Get the namespace
*/}}
{{- define "bambushain.namespace" -}}
{{- if .Values.branch -}}
{{- printf "%s-%s" .Values.namespace .Values.branch -}}
{{- else -}}
{{- .Values.namespace -}}
{{- end -}}
{{- end -}}