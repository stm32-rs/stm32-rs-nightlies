#[doc = "Register `SDMMC_ARGR` reader"]
pub type R = crate::R<SDMMC_ARGRrs>;
#[doc = "Register `SDMMC_ARGR` writer"]
pub type W = crate::W<SDMMC_ARGRrs>;
#[doc = "Field `CMDARG` reader - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
pub type CMDARG_R = crate::FieldReader<u32>;
#[doc = "Field `CMDARG` writer - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
pub type CMDARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument. These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0). Command argument sent to a card as part of a command message. If a command contains an argument, it must be loaded into this register before writing a command to the command register."]
    #[inline(always)]
    #[must_use]
    pub fn cmdarg(&mut self) -> CMDARG_W<SDMMC_ARGRrs> {
        CMDARG_W::new(self, 0)
    }
}
#[doc = "The SDMMC_ARGR register contains a 32-bit command argument, which is sent to a card as part of a command message.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_argr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_argr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_ARGRrs;
impl crate::RegisterSpec for SDMMC_ARGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_argr::R`](R) reader structure"]
impl crate::Readable for SDMMC_ARGRrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_argr::W`](W) writer structure"]
impl crate::Writable for SDMMC_ARGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_ARGR to value 0"]
impl crate::Resettable for SDMMC_ARGRrs {
    const RESET_VALUE: u32 = 0;
}
