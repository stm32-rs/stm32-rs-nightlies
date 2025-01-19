///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
/**Port x configuration pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEEDR0 {
    ///0: Low speed
    LowSpeed = 0,
    ///1: Medium speed
    MediumSpeed = 1,
    ///2: High speed
    HighSpeed = 2,
    ///3: Very high speed
    VeryHighSpeed = 3,
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
impl crate::IsEnum for OSPEEDR0 {}
///Field `OSPEEDR(0-4)` reader - Port x configuration pin %s
pub type OSPEEDR_R = crate::FieldReader<OSPEEDR0>;
impl OSPEEDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEEDR0 {
        match self.bits {
            0 => OSPEEDR0::LowSpeed,
            1 => OSPEEDR0::MediumSpeed,
            2 => OSPEEDR0::HighSpeed,
            3 => OSPEEDR0::VeryHighSpeed,
            _ => unreachable!(),
        }
    }
    ///Low speed
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OSPEEDR0::LowSpeed
    }
    ///Medium speed
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OSPEEDR0::MediumSpeed
    }
    ///High speed
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OSPEEDR0::HighSpeed
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        *self == OSPEEDR0::VeryHighSpeed
    }
}
///Field `OSPEEDR(0-4)` writer - Port x configuration pin %s
pub type OSPEEDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEEDR0, crate::Safe>;
impl<'a, REG> OSPEEDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR0::LowSpeed)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR0::MediumSpeed)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR0::HighSpeed)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEEDR0::VeryHighSpeed)
    }
}
impl R {
    ///Port x configuration pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OSPEEDR0` field.</div>
    #[inline(always)]
    pub fn ospeedr(&self, n: u8) -> OSPEEDR_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        OSPEEDR_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port x configuration pin (0-4)
    #[inline(always)]
    pub fn ospeedr_iter(&self) -> impl Iterator<Item = OSPEEDR_R> + '_ {
        (0..5).map(move |n| OSPEEDR_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR_R {
        OSPEEDR_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR_R {
        OSPEEDR_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeedr0", &self.ospeedr0())
            .field("ospeedr1", &self.ospeedr1())
            .field("ospeedr2", &self.ospeedr2())
            .field("ospeedr3", &self.ospeedr3())
            .field("ospeedr4", &self.ospeedr4())
            .finish()
    }
}
impl W {
    ///Port x configuration pin (0-4)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OSPEEDR0` field.</div>
    #[inline(always)]
    pub fn ospeedr(&mut self, n: u8) -> OSPEEDR_W<OSPEEDRrs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        OSPEEDR_W::new(self, n * 2)
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn ospeedr0(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn ospeedr1(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn ospeedr2(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn ospeedr3(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn ospeedr4(&mut self) -> OSPEEDR_W<OSPEEDRrs> {
        OSPEEDR_W::new(self, 8)
    }
}
/**GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#GPIOE:OSPEEDR)*/
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`ospeedr::R`](R) reader structure
impl crate::Readable for OSPEEDRrs {}
///`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OSPEEDR to value 0xc0
impl crate::Resettable for OSPEEDRrs {
    const RESET_VALUE: u32 = 0xc0;
}
