class ExamplesController < ApplicationController
  before_action :ensure_example, only: %i[update destroy]

  def create
    example = Example.create(day_id: params[:day_id], **example_params)
    if example.persisted?
      render(json: {id: example.id})
    else
      head(:unprocessable_entity)
    end
  end

  def update
    if example.update(**example_params)
      head(:ok)
    else
      head(:unprocessable_entity)
    end
  end

  def destroy
    example.destroy

    head(:ok)
  end

  protected

  def example
    return @example if defined?(@example)

    @example = Example.find_by(day_id: params[:day_id], id: params[:id])
  end

  private

  def ensure_example
    return if example.present?

    head(:not_found)
  end

  def example_params
    return {} unless params.key?(:example)

    params.require(:example).permit(:input, :expected_output).to_h.symbolize_keys
  end
end
