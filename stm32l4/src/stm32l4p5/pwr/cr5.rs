#[doc = "Register `CR5` reader"]
pub type R = crate::R<CR5rs>;
#[doc = "Register `CR5` writer"]
pub type W = crate::W<CR5rs>;
#[doc = "Main regulator Range 1 mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R1MODE {
    #[doc = "0: Main regulator in Range 1 boost mode"]
    BoostMode = 0,
    #[doc = "1: Main regulator in Range 1 normal mode"]
    NormalMode = 1,
}
impl From<R1MODE> for bool {
    #[inline(always)]
    fn from(variant: R1MODE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `R1MODE` reader - Main regulator Range 1 mode"]
pub type R1MODE_R = crate::BitReader<R1MODE>;
impl R1MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> R1MODE {
        match self.bits {
            false => R1MODE::BoostMode,
            true => R1MODE::NormalMode,
        }
    }
    #[doc = "Main regulator in Range 1 boost mode"]
    #[inline(always)]
    pub fn is_boost_mode(&self) -> bool {
        *self == R1MODE::BoostMode
    }
    #[doc = "Main regulator in Range 1 normal mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == R1MODE::NormalMode
    }
}
#[doc = "Field `R1MODE` writer - Main regulator Range 1 mode"]
pub type R1MODE_W<'a, REG> = crate::BitWriter<'a, REG, R1MODE>;
impl<'a, REG> R1MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Main regulator in Range 1 boost mode"]
    #[inline(always)]
    pub fn boost_mode(self) -> &'a mut crate::W<REG> {
        self.variant(R1MODE::BoostMode)
    }
    #[doc = "Main regulator in Range 1 normal mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(R1MODE::NormalMode)
    }
}
impl R {
    #[doc = "Bit 8 - Main regulator Range 1 mode"]
    #[inline(always)]
    pub fn r1mode(&self) -> R1MODE_R {
        R1MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Main regulator Range 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn r1mode(&mut self) -> R1MODE_W<CR5rs> {
        R1MODE_W::new(self, 8)
    }
}
#[doc = "control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR5rs;
impl crate::RegisterSpec for CR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr5::R`](R) reader structure"]
impl crate::Readable for CR5rs {}
#[doc = "`write(|w| ..)` method takes [`cr5::W`](W) writer structure"]
impl crate::Writable for CR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR5 to value 0x0100"]
impl crate::Resettable for CR5rs {
    const RESET_VALUE: u32 = 0x0100;
}
