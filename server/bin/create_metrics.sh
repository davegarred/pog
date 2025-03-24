#!/bin/bash
# Creates custom GCP log metrics for each of the metric log counter lines
metrics_list=(help \
    admin-help \
    admin-set_user_initiate \
    admin-welcome_channel \
    set_user_modal_response \
    initiate_bet \
    list_bets \
    pay_bet \
    attendance \
    whois \
    add_wager_modal_response
)
for metric in "${metrics_list[@]}";do
    METRIC_FILTER=$(cat metric-filter.txt | sed -e "s/{POG_COUNTER}/POG_COUNTER:${metric}/")
    METRIC_DESC="Counter metric for POG_COUNTER:${metric}"
    METRIC_NAME="POG_COUNTER-${metric}"
    echo "creating metric: ${METRIC_NAME}"
    gcloud logging metrics create \
      "${METRIC_NAME}" \
      --description="${METRIC_DESC}" \
      --log-filter="${METRIC_FILTER}"
done

