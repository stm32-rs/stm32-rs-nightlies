///Register `SAI_ADR` reader
pub type R = crate::R<SAI_ADRrs>;
///Register `SAI_ADR` writer
pub type W = crate::W<SAI_ADRrs>;
///Field `DATA` reader - DATA
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - DATA
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - DATA
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAI_ADR")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - DATA
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<SAI_ADRrs> {
        DATA_W::new(self, 0)
    }
}
/**Data register

You can [`read`](crate::Reg::read) this register and get [`sai_adr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_adr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#SAI1:SAI_ADR)*/
pub struct SAI_ADRrs;
impl crate::RegisterSpec for SAI_ADRrs {
    type Ux = u32;
}
///`read()` method returns [`sai_adr::R`](R) reader structure
impl crate::Readable for SAI_ADRrs {}
///`write(|w| ..)` method takes [`sai_adr::W`](W) writer structure
impl crate::Writable for SAI_ADRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAI_ADR to value 0
impl crate::Resettable for SAI_ADRrs {
    const RESET_VALUE: u32 = 0;
}
