#[doc = "Register `PARAM` reader"]
pub type R = crate::R<PARAMrs>;
#[doc = "Register `PARAM` writer"]
pub type W = crate::W<PARAMrs>;
#[doc = "Field `P` reader - Input parameter P"]
pub type P_R = crate::FieldReader;
#[doc = "Field `P` writer - Input parameter P"]
pub type P_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Q` reader - Input parameter Q"]
pub type Q_R = crate::FieldReader;
#[doc = "Field `Q` writer - Input parameter Q"]
pub type Q_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R` reader - Input parameter R"]
pub type R_R = crate::FieldReader;
#[doc = "Field `R` writer - Input parameter R"]
pub type R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FUNC` reader - Function"]
pub type FUNC_R = crate::FieldReader;
#[doc = "Field `FUNC` writer - Function"]
pub type FUNC_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `START` reader - Enable execution"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Enable execution"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Input parameter P"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Input parameter Q"]
    #[inline(always)]
    pub fn q(&self) -> Q_R {
        Q_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Input parameter R"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Function"]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Enable execution"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input parameter P"]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<PARAMrs> {
        P_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Input parameter Q"]
    #[inline(always)]
    #[must_use]
    pub fn q(&mut self) -> Q_W<PARAMrs> {
        Q_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Input parameter R"]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> R_W<PARAMrs> {
        R_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - Function"]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<PARAMrs> {
        FUNC_W::new(self, 24)
    }
    #[doc = "Bit 31 - Enable execution"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<PARAMrs> {
        START_W::new(self, 31)
    }
}
#[doc = "Parameter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`param::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`param::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PARAMrs;
impl crate::RegisterSpec for PARAMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`param::R`](R) reader structure"]
impl crate::Readable for PARAMrs {}
#[doc = "`write(|w| ..)` method takes [`param::W`](W) writer structure"]
impl crate::Writable for PARAMrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PARAM to value 0"]
impl crate::Resettable for PARAMrs {
    const RESET_VALUE: u32 = 0;
}
