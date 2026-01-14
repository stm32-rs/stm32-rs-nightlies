///Register `DINR27` reader
pub type R = crate::R<DINR27rs>;
///Field `DIN27` reader - Input data received from MDIO Master during write frames
pub type DIN27_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din27(&self) -> DIN27_R {
        DIN27_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DINR27")
            .field("din27", &self.din27())
            .finish()
    }
}
/**MDIOS input data register 27

You can [`read`](crate::Reg::read) this register and get [`dinr27::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#MDIOS:DINR27)*/
pub struct DINR27rs;
impl crate::RegisterSpec for DINR27rs {
    type Ux = u32;
}
///`read()` method returns [`dinr27::R`](R) reader structure
impl crate::Readable for DINR27rs {}
///`reset()` method sets DINR27 to value 0
impl crate::Resettable for DINR27rs {}
