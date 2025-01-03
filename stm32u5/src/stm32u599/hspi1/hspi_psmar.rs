///Register `HSPI_PSMAR` reader
pub type R = crate::R<HSPI_PSMARrs>;
///Register `HSPI_PSMAR` writer
pub type W = crate::W<HSPI_PSMARrs>;
///Field `MATCH` reader - 31: 0\]: Status match Value to be compared with the masked status register to get a match
pub type MATCH_R = crate::FieldReader<u32>;
///Field `MATCH` writer - 31: 0\]: Status match Value to be compared with the masked status register to get a match
pub type MATCH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - 31: 0\]: Status match Value to be compared with the masked status register to get a match
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSPI_PSMAR")
            .field("match_", &self.match_())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - 31: 0\]: Status match Value to be compared with the masked status register to get a match
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W<HSPI_PSMARrs> {
        MATCH_W::new(self, 0)
    }
}
/**HSPI polling status match register

You can [`read`](crate::Reg::read) this register and get [`hspi_psmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hspi_psmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#HSPI1:HSPI_PSMAR)*/
pub struct HSPI_PSMARrs;
impl crate::RegisterSpec for HSPI_PSMARrs {
    type Ux = u32;
}
///`read()` method returns [`hspi_psmar::R`](R) reader structure
impl crate::Readable for HSPI_PSMARrs {}
///`write(|w| ..)` method takes [`hspi_psmar::W`](W) writer structure
impl crate::Writable for HSPI_PSMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HSPI_PSMAR to value 0
impl crate::Resettable for HSPI_PSMARrs {
    const RESET_VALUE: u32 = 0;
}
