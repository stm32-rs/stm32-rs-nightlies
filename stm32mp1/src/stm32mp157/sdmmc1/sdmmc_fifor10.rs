#[doc = "Register `SDMMC_FIFOR10` reader"]
pub type R = crate::R<SDMMC_FIFOR10rs>;
#[doc = "Register `SDMMC_FIFOR10` writer"]
pub type W = crate::W<SDMMC_FIFOR10rs>;
#[doc = "Field `FIFODATA` reader - FIFODATA"]
pub type FIFODATA_R = crate::FieldReader<u32>;
#[doc = "Field `FIFODATA` writer - FIFODATA"]
pub type FIFODATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFODATA"]
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFODATA"]
    #[inline(always)]
    #[must_use]
    pub fn fifodata(&mut self) -> FIFODATA_W<SDMMC_FIFOR10rs> {
        FIFODATA_W::new(self, 0)
    }
}
#[doc = "The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers. The FIFOs contain 16 entries on sequential addresses. This allows the CPU to use its load and store multiple operands to read from/write to the FIFO. The FIFO register interface takes care of correct data alignment inside the FIFO, the FIFO register address used by the CPU does matter. When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_fifor10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_fifor10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_FIFOR10rs;
impl crate::RegisterSpec for SDMMC_FIFOR10rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_fifor10::R`](R) reader structure"]
impl crate::Readable for SDMMC_FIFOR10rs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_fifor10::W`](W) writer structure"]
impl crate::Writable for SDMMC_FIFOR10rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_FIFOR10 to value 0"]
impl crate::Resettable for SDMMC_FIFOR10rs {
    const RESET_VALUE: u32 = 0;
}
