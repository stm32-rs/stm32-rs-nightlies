///Register `AWHTR` reader
pub type R = crate::R<AWHTRrs>;
///Register `AWHTR` writer
pub type W = crate::W<AWHTRrs>;
/**Break signal assignment to analog watchdog high threshold event

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKAWH0 {
    ///0: Break i signal is not assigned to an analog watchdog high threshold event
    NotAssigned = 0,
    ///1: Break i signal is assigned to an analog watchdog high threshold event
    Assigned = 1,
}
impl From<BKAWH0> for bool {
    #[inline(always)]
    fn from(variant: BKAWH0) -> Self {
        variant as u8 != 0
    }
}
///Field `BKAWH(0-3)` reader - Break signal assignment to analog watchdog high threshold event
pub type BKAWH_R = crate::BitReader<BKAWH0>;
impl BKAWH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKAWH0 {
        match self.bits {
            false => BKAWH0::NotAssigned,
            true => BKAWH0::Assigned,
        }
    }
    ///Break i signal is not assigned to an analog watchdog high threshold event
    #[inline(always)]
    pub fn is_not_assigned(&self) -> bool {
        *self == BKAWH0::NotAssigned
    }
    ///Break i signal is assigned to an analog watchdog high threshold event
    #[inline(always)]
    pub fn is_assigned(&self) -> bool {
        *self == BKAWH0::Assigned
    }
}
///Field `BKAWH(0-3)` writer - Break signal assignment to analog watchdog high threshold event
pub type BKAWH_W<'a, REG> = crate::BitWriter<'a, REG, BKAWH0>;
impl<'a, REG> BKAWH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break i signal is not assigned to an analog watchdog high threshold event
    #[inline(always)]
    pub fn not_assigned(self) -> &'a mut crate::W<REG> {
        self.variant(BKAWH0::NotAssigned)
    }
    ///Break i signal is assigned to an analog watchdog high threshold event
    #[inline(always)]
    pub fn assigned(self) -> &'a mut crate::W<REG> {
        self.variant(BKAWH0::Assigned)
    }
}
///Field `AWHT` reader - Analog watchdog high threshold
pub type AWHT_R = crate::FieldReader<u32>;
///Field `AWHT` writer - Analog watchdog high threshold
pub type AWHT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32, crate::Safe>;
impl R {
    ///Break signal assignment to analog watchdog high threshold event
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BKAWH0` field.</div>
    #[inline(always)]
    pub fn bkawh(&self, n: u8) -> BKAWH_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        BKAWH_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh_iter(&self) -> impl Iterator<Item = BKAWH_R> + '_ {
        (0..4).map(move |n| BKAWH_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh0(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh1(&self) -> BKAWH_R {
        BKAWH_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh2(&self) -> BKAWH_R {
        BKAWH_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh3(&self) -> BKAWH_R {
        BKAWH_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:31 - Analog watchdog high threshold
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWHTR")
            .field("awht", &self.awht())
            .field("bkawh0", &self.bkawh0())
            .field("bkawh1", &self.bkawh1())
            .field("bkawh2", &self.bkawh2())
            .field("bkawh3", &self.bkawh3())
            .finish()
    }
}
impl W {
    ///Break signal assignment to analog watchdog high threshold event
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BKAWH0` field.</div>
    #[inline(always)]
    pub fn bkawh(&mut self, n: u8) -> BKAWH_W<'_, AWHTRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        BKAWH_W::new(self, n)
    }
    ///Bit 0 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh0(&mut self) -> BKAWH_W<'_, AWHTRrs> {
        BKAWH_W::new(self, 0)
    }
    ///Bit 1 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh1(&mut self) -> BKAWH_W<'_, AWHTRrs> {
        BKAWH_W::new(self, 1)
    }
    ///Bit 2 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh2(&mut self) -> BKAWH_W<'_, AWHTRrs> {
        BKAWH_W::new(self, 2)
    }
    ///Bit 3 - Break signal assignment to analog watchdog high threshold event
    #[inline(always)]
    pub fn bkawh3(&mut self) -> BKAWH_W<'_, AWHTRrs> {
        BKAWH_W::new(self, 3)
    }
    ///Bits 8:31 - Analog watchdog high threshold
    #[inline(always)]
    pub fn awht(&mut self) -> AWHT_W<'_, AWHTRrs> {
        AWHT_W::new(self, 8)
    }
}
/**DFSDM analog watchdog high threshold register

You can [`read`](crate::Reg::read) this register and get [`awhtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awhtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AWHTRrs;
impl crate::RegisterSpec for AWHTRrs {
    type Ux = u32;
}
///`read()` method returns [`awhtr::R`](R) reader structure
impl crate::Readable for AWHTRrs {}
///`write(|w| ..)` method takes [`awhtr::W`](W) writer structure
impl crate::Writable for AWHTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWHTR to value 0
impl crate::Resettable for AWHTRrs {}
