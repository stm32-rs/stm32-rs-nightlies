#[doc = "Register `DDRCTRL_INIT1` reader"]
pub type R = crate::R<DDRCTRL_INIT1rs>;
#[doc = "Register `DDRCTRL_INIT1` writer"]
pub type W = crate::W<DDRCTRL_INIT1rs>;
#[doc = "Field `PRE_OCD_X32` reader - PRE_OCD_X32"]
pub type PRE_OCD_X32_R = crate::FieldReader;
#[doc = "Field `PRE_OCD_X32` writer - PRE_OCD_X32"]
pub type PRE_OCD_X32_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DRAM_RSTN_X1024` reader - DRAM_RSTN_X1024"]
pub type DRAM_RSTN_X1024_R = crate::FieldReader<u16>;
#[doc = "Field `DRAM_RSTN_X1024` writer - DRAM_RSTN_X1024"]
pub type DRAM_RSTN_X1024_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:3 - PRE_OCD_X32"]
    #[inline(always)]
    pub fn pre_ocd_x32(&self) -> PRE_OCD_X32_R {
        PRE_OCD_X32_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - DRAM_RSTN_X1024"]
    #[inline(always)]
    pub fn dram_rstn_x1024(&self) -> DRAM_RSTN_X1024_R {
        DRAM_RSTN_X1024_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - PRE_OCD_X32"]
    #[inline(always)]
    #[must_use]
    pub fn pre_ocd_x32(&mut self) -> PRE_OCD_X32_W<DDRCTRL_INIT1rs> {
        PRE_OCD_X32_W::new(self, 0)
    }
    #[doc = "Bits 16:24 - DRAM_RSTN_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn dram_rstn_x1024(&mut self) -> DRAM_RSTN_X1024_W<DDRCTRL_INIT1rs> {
        DRAM_RSTN_X1024_W::new(self, 16)
    }
}
#[doc = "DDRCTRL SDRAM initialization register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrctrl_init1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrctrl_init1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRCTRL_INIT1rs;
impl crate::RegisterSpec for DDRCTRL_INIT1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrctrl_init1::R`](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT1rs {}
#[doc = "`write(|w| ..)` method takes [`ddrctrl_init1::W`](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRCTRL_INIT1 to value 0"]
impl crate::Resettable for DDRCTRL_INIT1rs {
    const RESET_VALUE: u32 = 0;
}
