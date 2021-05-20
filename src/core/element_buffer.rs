use crate::context::{consts, Context};
use crate::core::{ElementBufferDataType, Error};

///
/// A buffer containing indices for rendering, see for example [draw_elements](crate::Program::draw_elements).
/// Also known as an index buffer.
///
pub struct ElementBuffer<T: ElementBufferDataType> {
    context: Context,
    id: crate::context::Buffer,
    count: usize,
    _dummy: T,
}

impl<T: ElementBufferDataType> ElementBuffer<T> {
    ///
    /// Creates a new element buffer and fills it with the given indices.
    ///
    pub fn new(context: &Context, data: &[T]) -> Result<ElementBuffer<T>, Error> {
        let id = context.create_buffer().unwrap();
        let mut buffer = ElementBuffer {
            context: context.clone(),
            id,
            count: 0,
            _dummy: T::default(),
        };
        if data.len() > 0 {
            buffer.fill_with(data);
        }
        Ok(buffer)
    }

    pub(crate) fn new_from_indices(
        context: &Context,
        indices: &crate::Indices,
    ) -> Result<ElementBuffer<T>, Error> {
        let id = context.create_buffer().unwrap();
        Ok(match indices {
            crate::Indices::U8(data) => {
                let mut buffer = ElementBuffer {
                    context: context.clone(),
                    id,
                    count: 0,
                    _dummy: T::default(),
                };
                if data.len() > 0 {
                    buffer.fill_with_internal(data);
                }
                buffer
            }
            crate::Indices::U16(data) => {
                let mut buffer = ElementBuffer {
                    context: context.clone(),
                    id,
                    count: 0,
                    _dummy: T::default(),
                };
                if data.len() > 0 {
                    buffer.fill_with_internal(data);
                }
                buffer
            }
            crate::Indices::U32(data) => {
                let mut buffer = ElementBuffer {
                    context: context.clone(),
                    id,
                    count: 0,
                    _dummy: T::default(),
                };
                if data.len() > 0 {
                    buffer.fill_with_internal(data);
                }
                buffer
            }
        })
    }

    ///
    /// Fills the buffer with the given indices.
    ///
    pub fn fill_with(&mut self, data: &[T]) {
        self.fill_with_internal(data);
    }

    fn fill_with_internal<Q: ElementBufferDataType>(&mut self, data: &[Q]) {
        self.bind();
        Q::buffer_data(
            &self.context,
            consts::ELEMENT_ARRAY_BUFFER,
            data,
            consts::STATIC_DRAW,
        );
        self.context.unbind_buffer(consts::ELEMENT_ARRAY_BUFFER);
        self.count = data.len();
    }

    ///
    /// The number of elements in the buffer.
    ///
    pub fn count(&self) -> usize {
        self.count
    }

    pub(crate) fn bind(&self) {
        self.context
            .bind_buffer(consts::ELEMENT_ARRAY_BUFFER, &self.id);
    }
}

impl<T: ElementBufferDataType> Drop for ElementBuffer<T> {
    fn drop(&mut self) {
        self.context.delete_buffer(&self.id);
    }
}
