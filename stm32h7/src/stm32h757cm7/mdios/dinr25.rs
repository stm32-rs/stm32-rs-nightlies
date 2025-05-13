///Register `DINR25` reader
pub type R = crate::R<DINR25rs>;
///Field `DIN25` reader - Input data received from MDIO Master during write frames
pub type DIN25_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din25(&self) -> DIN25_R {
        DIN25_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR25")
            .field("din25", &self.din25())
            .finish()
    }
}
/**MDIOS input data register 25

You can [`read`](crate::Reg::read) this register and get [`dinr25::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#MDIOS:DINR25)*/
pub struct DINR25rs;
impl crate::RegisterSpec for DINR25rs {
    type Ux = u32;
}
///`read()` method returns [`dinr25::R`](R) reader structure
impl crate::Readable for DINR25rs {}
///`reset()` method sets DINR25 to value 0
impl crate::Resettable for DINR25rs {}
