#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNTrs>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CNTrs>;
#[doc = "Field `CNT_bit0` reader - CNT"]
pub type CNT_BIT0_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_bit0` writer - CNT"]
pub type CNT_BIT0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `UIFCPY` reader - UIFCPY or Res"]
pub type UIFCPY_R = crate::BitReader;
#[doc = "Field `UIFCPY` writer - UIFCPY or Res"]
pub type UIFCPY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - CNT"]
    #[inline(always)]
    pub fn cnt_bit0(&self) -> CNT_BIT0_R {
        CNT_BIT0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - UIFCPY or Res"]
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CNT"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_bit0(&mut self) -> CNT_BIT0_W<CNTrs> {
        CNT_BIT0_W::new(self, 0)
    }
    #[doc = "Bit 31 - UIFCPY or Res"]
    #[inline(always)]
    #[must_use]
    pub fn uifcpy(&mut self) -> UIFCPY_W<CNTrs> {
        UIFCPY_W::new(self, 31)
    }
}
#[doc = "counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTrs;
impl crate::RegisterSpec for CNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNTrs {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNTrs {
    const RESET_VALUE: u32 = 0;
}
