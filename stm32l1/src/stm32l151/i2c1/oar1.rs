///Register `OAR1` reader
pub type R = crate::R<OAR1rs>;
///Register `OAR1` writer
pub type W = crate::W<OAR1rs>;
///Field `ADD` reader - Interface address
pub type ADD_R = crate::FieldReader<u16>;
///Field `ADD` writer - Interface address
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
/**ADDMODE

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDMODE {
    ///0: 7-bit slave address
    Add7 = 0,
    ///1: 10-bit slave address
    Add10 = 1,
}
impl From<ADDMODE> for bool {
    #[inline(always)]
    fn from(variant: ADDMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `ADDMODE` reader - ADDMODE
pub type ADDMODE_R = crate::BitReader<ADDMODE>;
impl ADDMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADDMODE {
        match self.bits {
            false => ADDMODE::Add7,
            true => ADDMODE::Add10,
        }
    }
    ///7-bit slave address
    #[inline(always)]
    pub fn is_add7(&self) -> bool {
        *self == ADDMODE::Add7
    }
    ///10-bit slave address
    #[inline(always)]
    pub fn is_add10(&self) -> bool {
        *self == ADDMODE::Add10
    }
}
///Field `ADDMODE` writer - ADDMODE
pub type ADDMODE_W<'a, REG> = crate::BitWriter<'a, REG, ADDMODE>;
impl<'a, REG> ADDMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///7-bit slave address
    #[inline(always)]
    pub fn add7(self) -> &'a mut crate::W<REG> {
        self.variant(ADDMODE::Add7)
    }
    ///10-bit slave address
    #[inline(always)]
    pub fn add10(self) -> &'a mut crate::W<REG> {
        self.variant(ADDMODE::Add10)
    }
}
impl R {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(self.bits & 0x03ff)
    }
    ///Bit 15 - ADDMODE
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR1")
            .field("addmode", &self.addmode())
            .field("add", &self.add())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<'_, OAR1rs> {
        ADD_W::new(self, 0)
    }
    ///Bit 15 - ADDMODE
    #[inline(always)]
    pub fn addmode(&mut self) -> ADDMODE_W<'_, OAR1rs> {
        ADDMODE_W::new(self, 15)
    }
}
/**OAR1

You can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#I2C1:OAR1)*/
pub struct OAR1rs;
impl crate::RegisterSpec for OAR1rs {
    type Ux = u16;
}
///`read()` method returns [`oar1::R`](R) reader structure
impl crate::Readable for OAR1rs {}
///`write(|w| ..)` method takes [`oar1::W`](W) writer structure
impl crate::Writable for OAR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OAR1 to value 0
impl crate::Resettable for OAR1rs {}
