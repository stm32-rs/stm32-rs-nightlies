#[doc = "Register `RCC_MP_IWDGFZSETR` reader"]
pub type R = crate::R<RCC_MP_IWDGFZSETRrs>;
#[doc = "Register `RCC_MP_IWDGFZSETR` writer"]
pub type W = crate::W<RCC_MP_IWDGFZSETRrs>;
#[doc = "Field `FZ_IWDG1` reader - FZ_IWDG1"]
pub type FZ_IWDG1_R = crate::BitReader;
#[doc = "Field `FZ_IWDG1` writer - FZ_IWDG1"]
pub type FZ_IWDG1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FZ_IWDG2` reader - FZ_IWDG2"]
pub type FZ_IWDG2_R = crate::BitReader;
#[doc = "Field `FZ_IWDG2` writer - FZ_IWDG2"]
pub type FZ_IWDG2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FZ_IWDG1"]
    #[inline(always)]
    pub fn fz_iwdg1(&self) -> FZ_IWDG1_R {
        FZ_IWDG1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FZ_IWDG2"]
    #[inline(always)]
    pub fn fz_iwdg2(&self) -> FZ_IWDG2_R {
        FZ_IWDG2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FZ_IWDG1"]
    #[inline(always)]
    #[must_use]
    pub fn fz_iwdg1(&mut self) -> FZ_IWDG1_W<RCC_MP_IWDGFZSETRrs> {
        FZ_IWDG1_W::new(self, 0)
    }
    #[doc = "Bit 1 - FZ_IWDG2"]
    #[inline(always)]
    #[must_use]
    pub fn fz_iwdg2(&mut self) -> FZ_IWDG2_W<RCC_MP_IWDGFZSETRrs> {
        FZ_IWDG2_W::new(self, 1)
    }
}
#[doc = "This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_iwdgfzsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_iwdgfzsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_IWDGFZSETRrs;
impl crate::RegisterSpec for RCC_MP_IWDGFZSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_iwdgfzsetr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_IWDGFZSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_iwdgfzsetr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_IWDGFZSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_IWDGFZSETR to value 0"]
impl crate::Resettable for RCC_MP_IWDGFZSETRrs {
    const RESET_VALUE: u32 = 0;
}
