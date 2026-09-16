///Register `OPTCR2` reader
pub type R = crate::R<OPTCR2rs>;
///Register `OPTCR2` writer
pub type W = crate::W<OPTCR2rs>;
/**PCROP option byte

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCROP0 {
    ///0: PCROP protection inactive on sector %s
    Inactive = 0,
    ///1: PCROP protection active on sector %s
    Active = 1,
}
impl From<PCROP0> for bool {
    #[inline(always)]
    fn from(variant: PCROP0) -> Self {
        variant as u8 != 0
    }
}
///Field `PCROP(0-7)` reader - PCROP option byte
pub type PCROP_R = crate::BitReader<PCROP0>;
impl PCROP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PCROP0 {
        match self.bits {
            false => PCROP0::Inactive,
            true => PCROP0::Active,
        }
    }
    ///PCROP protection inactive on sector %s
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == PCROP0::Inactive
    }
    ///PCROP protection active on sector %s
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == PCROP0::Active
    }
}
///Field `PCROP(0-7)` writer - PCROP option byte
pub type PCROP_W<'a, REG> = crate::BitWriter<'a, REG, PCROP0>;
impl<'a, REG> PCROP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PCROP protection inactive on sector %s
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(PCROP0::Inactive)
    }
    ///PCROP protection active on sector %s
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(PCROP0::Active)
    }
}
///Field `PCROP_RDP` reader - PCROP zone preserved when RDP level decreased
pub use PCROP_R as PCROP_RDP_R;
///Field `PCROP_RDP` writer - PCROP zone preserved when RDP level decreased
pub use PCROP_W as PCROP_RDP_W;
impl R {
    ///PCROP option byte
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PCROP0` field.</div>
    #[inline(always)]
    pub fn pcrop(&self, n: u8) -> PCROP_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PCROP_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///PCROP option byte
    #[inline(always)]
    pub fn pcrop_iter(&self) -> impl Iterator<Item = PCROP_R> + '_ {
        (0..8).map(move |n| PCROP_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - PCROP option byte
    #[inline(always)]
    pub fn pcrop0(&self) -> PCROP_R {
        PCROP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PCROP option byte
    #[inline(always)]
    pub fn pcrop1(&self) -> PCROP_R {
        PCROP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PCROP option byte
    #[inline(always)]
    pub fn pcrop2(&self) -> PCROP_R {
        PCROP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PCROP option byte
    #[inline(always)]
    pub fn pcrop3(&self) -> PCROP_R {
        PCROP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PCROP option byte
    #[inline(always)]
    pub fn pcrop4(&self) -> PCROP_R {
        PCROP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PCROP option byte
    #[inline(always)]
    pub fn pcrop5(&self) -> PCROP_R {
        PCROP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PCROP option byte
    #[inline(always)]
    pub fn pcrop6(&self) -> PCROP_R {
        PCROP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PCROP option byte
    #[inline(always)]
    pub fn pcrop7(&self) -> PCROP_R {
        PCROP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 31 - PCROP zone preserved when RDP level decreased
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTCR2")
            .field("pcrop0", &self.pcrop0())
            .field("pcrop1", &self.pcrop1())
            .field("pcrop2", &self.pcrop2())
            .field("pcrop3", &self.pcrop3())
            .field("pcrop4", &self.pcrop4())
            .field("pcrop5", &self.pcrop5())
            .field("pcrop6", &self.pcrop6())
            .field("pcrop7", &self.pcrop7())
            .field("pcrop_rdp", &self.pcrop_rdp())
            .finish()
    }
}
impl W {
    ///PCROP option byte
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `PCROP0` field.</div>
    #[inline(always)]
    pub fn pcrop(&mut self, n: u8) -> PCROP_W<'_, OPTCR2rs> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        PCROP_W::new(self, n)
    }
    ///Bit 0 - PCROP option byte
    #[inline(always)]
    pub fn pcrop0(&mut self) -> PCROP_W<'_, OPTCR2rs> {
        PCROP_W::new(self, 0)
    }
    ///Bit 1 - PCROP option byte
    #[inline(always)]
    pub fn pcrop1(&mut self) -> PCROP_W<'_, OPTCR2rs> {
        PCROP_W::new(self, 1)
    }
    ///Bit 2 - PCROP option byte
    #[inline(always)]
    pub fn pcrop2(&mut self) -> PCROP_W<'_, OPTCR2rs> {
        PCROP_W::new(self, 2)
    }
    ///Bit 3 - PCROP option byte
    #[inline(always)]
    pub fn pcrop3(&mut self) -> PCROP_W<'_, OPTCR2rs> {
        PCROP_W::new(self, 3)
    }
    ///Bit 4 - PCROP option byte
    #[inline(always)]
    pub fn pcrop4(&mut self) -> PCROP_W<'_, OPTCR2rs> {
        PCROP_W::new(self, 4)
    }
    ///Bit 5 - PCROP option byte
    #[inline(always)]
    pub fn pcrop5(&mut self) -> PCROP_W<'_, OPTCR2rs> {
        PCROP_W::new(self, 5)
    }
    ///Bit 6 - PCROP option byte
    #[inline(always)]
    pub fn pcrop6(&mut self) -> PCROP_W<'_, OPTCR2rs> {
        PCROP_W::new(self, 6)
    }
    ///Bit 7 - PCROP option byte
    #[inline(always)]
    pub fn pcrop7(&mut self) -> PCROP_W<'_, OPTCR2rs> {
        PCROP_W::new(self, 7)
    }
    ///Bit 31 - PCROP zone preserved when RDP level decreased
    #[inline(always)]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<'_, OPTCR2rs> {
        PCROP_RDP_W::new(self, 31)
    }
}
/**Flash option control register

You can [`read`](crate::Reg::read) this register and get [`optcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F732.html#FLASH:OPTCR2)*/
pub struct OPTCR2rs;
impl crate::RegisterSpec for OPTCR2rs {
    type Ux = u32;
}
///`read()` method returns [`optcr2::R`](R) reader structure
impl crate::Readable for OPTCR2rs {}
///`write(|w| ..)` method takes [`optcr2::W`](W) writer structure
impl crate::Writable for OPTCR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OPTCR2 to value 0x8000_00ff
impl crate::Resettable for OPTCR2rs {
    const RESET_VALUE: u32 = 0x8000_00ff;
}
