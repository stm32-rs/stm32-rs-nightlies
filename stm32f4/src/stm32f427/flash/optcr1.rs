#[doc = "Register `OPTCR1` reader"]
pub type R = crate::R<OPTCR1rs>;
#[doc = "Register `OPTCR1` writer"]
pub type W = crate::W<OPTCR1rs>;
#[doc = "Field `nWRP` reader - Not write protect"]
pub type N_WRP_R = crate::FieldReader<u16>;
#[doc = "Field `nWRP` writer - Not write protect"]
pub type N_WRP_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 16:27 - Not write protect"]
    #[inline(always)]
    pub fn n_wrp(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - Not write protect"]
    #[inline(always)]
    #[must_use]
    pub fn n_wrp(&mut self) -> N_WRP_W<OPTCR1rs> {
        N_WRP_W::new(self, 16)
    }
}
#[doc = "Flash option control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`optcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`optcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTCR1rs;
impl crate::RegisterSpec for OPTCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optcr1::R`](R) reader structure"]
impl crate::Readable for OPTCR1rs {}
#[doc = "`write(|w| ..)` method takes [`optcr1::W`](W) writer structure"]
impl crate::Writable for OPTCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPTCR1 to value 0x0fff_0000"]
impl crate::Resettable for OPTCR1rs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
