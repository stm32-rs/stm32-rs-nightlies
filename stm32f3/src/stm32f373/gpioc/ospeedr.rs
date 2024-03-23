#[doc = "Register `OSPEEDR` reader"]
pub type R = crate::R<OSPEEDRrs>;
#[doc = "Register `OSPEEDR` writer"]
pub type W = crate::W<OSPEEDRrs>;
#[doc = "Port x configuration pin %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEEDR0 {
    #[doc = "0: Low speed"]
    LowSpeed = 0,
    #[doc = "1: Medium speed"]
    MediumSpeed = 1,
    #[doc = "3: High speed"]
    HighSpeed = 3,
}
impl From<OSPEEDR0> for u8 {
    #[inline(always)]
    fn from(variant: OSPEEDR0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEEDR0 {
    type Ux = u8;
}
#[doc = "Field `OSPEEDR(0-15)` reader - Port x configuration pin %s"]
pub type OSPEEDR_R = crate::FieldReader<OSPEEDR0>;
impl OSPEEDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OSPEEDR0> {
        match self.bits {
            0 => Some(OSPEEDR0::LowSpeed),
            1 => Some(OSPEEDR0::MediumSpeed),
            3 => Some(OSPEEDR0::HighSpeed),
            _ => None,
        }
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OSPEEDR0::LowSpeed
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OSPEEDR0::MediumSpeed
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OSPEEDR0::HighSpeed
    }
}
#[doc = "Field `OSPEEDR(0-15)` writer - Port x configuration pin %s"]
pub type OSPEEDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEEDR0>;
impl<'a, REG> OSPEEDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR0::LowSpeed)
    }
    #[doc = "Medium speed"]
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR0::MediumSpeed)
    }
    #[doc = "High speed"]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR0::HighSpeed)
    }
}
impl R {
    #[doc = "Port x configuration pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OSPEEDR0` field"]
    #[inline(always)]
    pub fn ospeedr(&self, n: u8) -> OSPEEDR_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OSPEEDR_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Port x configuration pin (0-15)"]
    #[inline(always)]
    pub fn ospeedr_iter(&self) -> impl Iterator<Item = OSPEEDR_R> + '_ {
        (0..16).map(move |n| OSPEEDR_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Port x configuration pin 0"]
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR_R {
        OSPEEDR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port x configuration pin 1"]
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port x configuration pin 2"]
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port x configuration pin 3"]
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port x configuration pin 4"]
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port x configuration pin 5"]
    #[inline(always)]
    pub fn ospeedr5(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port x configuration pin 6"]
    #[inline(always)]
    pub fn ospeedr6(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port x configuration pin 7"]
    #[inline(always)]
    pub fn ospeedr7(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port x configuration pin 8"]
    #[inline(always)]
    pub fn ospeedr8(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port x configuration pin 9"]
    #[inline(always)]
    pub fn ospeedr9(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port x configuration pin 10"]
    #[inline(always)]
    pub fn ospeedr10(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port x configuration pin 11"]
    #[inline(always)]
    pub fn ospeedr11(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port x configuration pin 12"]
    #[inline(always)]
    pub fn ospeedr12(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port x configuration pin 13"]
    #[inline(always)]
    pub fn ospeedr13(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port x configuration pin 14"]
    #[inline(always)]
    pub fn ospeedr14(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port x configuration pin 15"]
    #[inline(always)]
    pub fn ospeedr15(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Port x configuration pin (0-15)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `OSPEEDR0` field"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr(&mut self, n: u8) -> OSPEEDR_W<OSPEEDRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OSPEEDR_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - Port x configuration pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr0(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port x configuration pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr1(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port x configuration pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr2(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port x configuration pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr3(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port x configuration pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr4(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port x configuration pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr5(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port x configuration pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr6(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port x configuration pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr7(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port x configuration pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr8(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port x configuration pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr9(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port x configuration pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr10(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port x configuration pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr11(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port x configuration pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr12(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port x configuration pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr13(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port x configuration pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr14(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port x configuration pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn ospeedr15(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 30)
    }
}
#[doc = "GPIO port output speed register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospeedr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospeedr::R`](R) reader structure"]
impl crate::Readable for OSPEEDRrs {}
#[doc = "`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure"]
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPEEDR to value 0"]
impl crate::Resettable for OSPEEDRrs {
    const RESET_VALUE: u32 = 0;
}
