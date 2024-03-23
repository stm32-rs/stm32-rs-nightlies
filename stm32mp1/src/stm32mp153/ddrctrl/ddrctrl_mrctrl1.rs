#[doc = "Register `DDRCTRL_MRCTRL1` reader"]
pub type R = crate::R<DDRCTRL_MRCTRL1rs>;
#[doc = "Register `DDRCTRL_MRCTRL1` writer"]
pub type W = crate::W<DDRCTRL_MRCTRL1rs>;
#[doc = "Field `MR_DATA` reader - MR_DATA"]
pub type MR_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `MR_DATA` writer - MR_DATA"]
pub type MR_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MR_DATA"]
    #[inline(always)]
    pub fn mr_data(&self) -> MR_DATA_R {
        MR_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MR_DATA"]
    #[inline(always)]
    #[must_use]
    pub fn mr_data(&mut self) -> MR_DATA_W<DDRCTRL_MRCTRL1rs> {
        MR_DATA_W::new(self, 0)
    }
}
#[doc = "DDRCTRL mode register read/write control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_mrctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_mrctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_MRCTRL1rs;
impl crate::RegisterSpec for DDRCTRL_MRCTRL1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_mrctrl1::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_MRCTRL1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_mrctrl1::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_MRCTRL1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_MRCTRL1 to value 0"]
impl crate::Resettable for DDRCTRL_MRCTRL1rs {
    const RESET_VALUE: u32 = 0;
}
