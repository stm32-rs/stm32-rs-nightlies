///Register `STR%s` reader
pub type R = crate::R<STRrs>;
///Register `STR%s` writer
pub type W = crate::W<STRrs>;
///Field `STRSTDATA` reader - DAC Channel 1 Sawtooth reset value
pub type STRSTDATA_R = crate::FieldReader<u16>;
///Field `STRSTDATA` writer - DAC Channel 1 Sawtooth reset value
pub type STRSTDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16, crate::Safe>;
/**DAC Channel1 Sawtooth direction setting

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STDIR1 {
    ///0: Decrement
    Decrement = 0,
    ///1: Increment
    Increment = 1,
}
impl From<STDIR1> for bool {
    #[inline(always)]
    fn from(variant: STDIR1) -> Self {
        variant as u8 != 0
    }
}
///Field `STDIR` reader - DAC Channel1 Sawtooth direction setting
pub type STDIR_R = crate::BitReader<STDIR1>;
impl STDIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STDIR1 {
        match self.bits {
            false => STDIR1::Decrement,
            true => STDIR1::Increment,
        }
    }
    ///Decrement
    #[inline(always)]
    pub fn is_decrement(&self) -> bool {
        *self == STDIR1::Decrement
    }
    ///Increment
    #[inline(always)]
    pub fn is_increment(&self) -> bool {
        *self == STDIR1::Increment
    }
}
///Field `STDIR` writer - DAC Channel1 Sawtooth direction setting
pub type STDIR_W<'a, REG> = crate::BitWriter<'a, REG, STDIR1>;
impl<'a, REG> STDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Decrement
    #[inline(always)]
    pub fn decrement(self) -> &'a mut crate::W<REG> {
        self.variant(STDIR1::Decrement)
    }
    ///Increment
    #[inline(always)]
    pub fn increment(self) -> &'a mut crate::W<REG> {
        self.variant(STDIR1::Increment)
    }
}
///Field `STINCDATA` reader - DAC CH1 Sawtooth increment value (12.4 bit format)
pub type STINCDATA_R = crate::FieldReader<u16>;
///Field `STINCDATA` writer - DAC CH1 Sawtooth increment value (12.4 bit format)
pub type STINCDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    ///Bits 0:11 - DAC Channel 1 Sawtooth reset value
    #[inline(always)]
    pub fn strstdata(&self) -> STRSTDATA_R {
        STRSTDATA_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - DAC Channel1 Sawtooth direction setting
    #[inline(always)]
    pub fn stdir(&self) -> STDIR_R {
        STDIR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)
    #[inline(always)]
    pub fn stincdata(&self) -> STINCDATA_R {
        STINCDATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STR")
            .field("strstdata", &self.strstdata())
            .field("stdir", &self.stdir())
            .field("stincdata", &self.stincdata())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - DAC Channel 1 Sawtooth reset value
    #[inline(always)]
    pub fn strstdata(&mut self) -> STRSTDATA_W<'_, STRrs> {
        STRSTDATA_W::new(self, 0)
    }
    ///Bit 12 - DAC Channel1 Sawtooth direction setting
    #[inline(always)]
    pub fn stdir(&mut self) -> STDIR_W<'_, STRrs> {
        STDIR_W::new(self, 12)
    }
    ///Bits 16:31 - DAC CH1 Sawtooth increment value (12.4 bit format)
    #[inline(always)]
    pub fn stincdata(&mut self) -> STINCDATA_W<'_, STRrs> {
        STINCDATA_W::new(self, 16)
    }
}
/**Sawtooth register

You can [`read`](crate::Reg::read) this register and get [`str::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G441.html#DAC1:STR[1])*/
pub struct STRrs;
impl crate::RegisterSpec for STRrs {
    type Ux = u32;
}
///`read()` method returns [`str::R`](R) reader structure
impl crate::Readable for STRrs {}
///`write(|w| ..)` method takes [`str::W`](W) writer structure
impl crate::Writable for STRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STR%s to value 0
impl crate::Resettable for STRrs {}
