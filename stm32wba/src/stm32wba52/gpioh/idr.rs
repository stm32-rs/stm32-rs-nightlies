///Register `IDR` reader
pub type R = crate::R<IDRrs>;
///Field `ID3` reader - Port H input data I/O pin 3 This bit is read-only. It contain the input value of the corresponding I/O port. Access can be protected by GPIOH SEC3.
pub type ID3_R = crate::BitReader;
impl R {
    ///Bit 3 - Port H input data I/O pin 3 This bit is read-only. It contain the input value of the corresponding I/O port. Access can be protected by GPIOH SEC3.
    #[inline(always)]
    pub fn id3(&self) -> ID3_R {
        ID3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR").field("id3", &self.id3()).finish()
    }
}
/**GPIO port H input data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#GPIOH:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {}
