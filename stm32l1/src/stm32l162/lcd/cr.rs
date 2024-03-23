#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `LCDEN` reader - LCD controller enable"]
pub type LCDEN_R = crate::BitReader;
#[doc = "Field `LCDEN` writer - LCD controller enable"]
pub type LCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSEL` reader - Voltage source selection"]
pub type VSEL_R = crate::BitReader;
#[doc = "Field `VSEL` writer - Voltage source selection"]
pub type VSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUTY` reader - Duty selection"]
pub type DUTY_R = crate::FieldReader;
#[doc = "Field `DUTY` writer - Duty selection"]
pub type DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BIAS` reader - Bias selector"]
pub type BIAS_R = crate::FieldReader;
#[doc = "Field `BIAS` writer - Bias selector"]
pub type BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MUX_SEG` reader - Mux segment enable"]
pub type MUX_SEG_R = crate::BitReader;
#[doc = "Field `MUX_SEG` writer - Mux segment enable"]
pub type MUX_SEG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD controller enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage source selection"]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Duty selection"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - Bias selector"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Mux segment enable"]
    #[inline(always)]
    pub fn mux_seg(&self) -> MUX_SEG_R {
        MUX_SEG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD controller enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcden(&mut self) -> LCDEN_W<CRrs> {
        LCDEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Voltage source selection"]
    #[inline(always)]
    #[must_use]
    pub fn vsel(&mut self) -> VSEL_W<CRrs> {
        VSEL_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Duty selection"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<CRrs> {
        DUTY_W::new(self, 2)
    }
    #[doc = "Bits 5:6 - Bias selector"]
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BIAS_W<CRrs> {
        BIAS_W::new(self, 5)
    }
    #[doc = "Bit 7 - Mux segment enable"]
    #[inline(always)]
    #[must_use]
    pub fn mux_seg(&mut self) -> MUX_SEG_W<CRrs> {
        MUX_SEG_W::new(self, 7)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
