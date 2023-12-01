class DaysController < ApplicationController
  helper_method :days, :day, :preamble, :epilogue

  before_action :ensure_day, only: %i[show edit update]

  def index
  end

  def show
  end

  def edit
  end

  def update
    if day.update(**day_params)
      head(:ok)
    else
      head(:unprocessable_entity)
    end
  end

  protected

  def days
    @days ||= Day.all
  end

  def day
    return @day if defined?(@day)

    @day = Day.includes(:examples).find_by(id: params[:id])
  end

  def preamble
    I18n.t("solution.preamble")
  end

  def epilogue
    I18n.t("solution.epilogue")
  end

  private

  def ensure_day
    render(:not_found) unless day.present?
  end

  def day_params
    return {} unless params.key?(:day)

    params
      .require(:day)
      .permit(:title, :description, :solution)
      .to_h
      .symbolize_keys
  end
end
