///Register `AWLTR` reader
pub type R = crate::R<AWLTRrs>;
///Register `AWLTR` writer
pub type W = crate::W<AWLTRrs>;
/**Break signal assignment to analog watchdog low threshold event

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKAWL0 {
    ///0: Break i signal is not assigned to an analog watchdog low threshold event
    NotAssigned = 0,
    ///1: Break i signal is assigned to an analog watchdog low threshold event
    Assigned = 1,
}
impl From<BKAWL0> for bool {
    #[inline(always)]
    fn from(variant: BKAWL0) -> Self {
        variant as u8 != 0
    }
}
///Field `BKAWL(0-3)` reader - Break signal assignment to analog watchdog low threshold event
pub type BKAWL_R = crate::BitReader<BKAWL0>;
impl BKAWL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BKAWL0 {
        match self.bits {
            false => BKAWL0::NotAssigned,
            true => BKAWL0::Assigned,
        }
    }
    ///Break i signal is not assigned to an analog watchdog low threshold event
    #[inline(always)]
    pub fn is_not_assigned(&self) -> bool {
        *self == BKAWL0::NotAssigned
    }
    ///Break i signal is assigned to an analog watchdog low threshold event
    #[inline(always)]
    pub fn is_assigned(&self) -> bool {
        *self == BKAWL0::Assigned
    }
}
///Field `BKAWL(0-3)` writer - Break signal assignment to analog watchdog low threshold event
pub type BKAWL_W<'a, REG> = crate::BitWriter<'a, REG, BKAWL0>;
impl<'a, REG> BKAWL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break i signal is not assigned to an analog watchdog low threshold event
    #[inline(always)]
    pub fn not_assigned(self) -> &'a mut crate::W<REG> {
        self.variant(BKAWL0::NotAssigned)
    }
    ///Break i signal is assigned to an analog watchdog low threshold event
    #[inline(always)]
    pub fn assigned(self) -> &'a mut crate::W<REG> {
        self.variant(BKAWL0::Assigned)
    }
}
///Field `AWLT` reader - Analog watchdog low threshold
pub type AWLT_R = crate::FieldReader<u32>;
///Field `AWLT` writer - Analog watchdog low threshold
pub type AWLT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32, crate::Safe>;
impl R {
    ///Break signal assignment to analog watchdog low threshold event
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BKAWL0` field.</div>
    #[inline(always)]
    pub fn bkawl(&self, n: u8) -> BKAWL_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        BKAWL_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl_iter(&self) -> impl Iterator<Item = BKAWL_R> + '_ {
        (0..4).map(move |n| BKAWL_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl0(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl1(&self) -> BKAWL_R {
        BKAWL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl2(&self) -> BKAWL_R {
        BKAWL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl3(&self) -> BKAWL_R {
        BKAWL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:31 - Analog watchdog low threshold
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWLTR")
            .field("awlt", &self.awlt())
            .field("bkawl0", &self.bkawl0())
            .field("bkawl1", &self.bkawl1())
            .field("bkawl2", &self.bkawl2())
            .field("bkawl3", &self.bkawl3())
            .finish()
    }
}
impl W {
    ///Break signal assignment to analog watchdog low threshold event
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `BKAWL0` field.</div>
    #[inline(always)]
    pub fn bkawl(&mut self, n: u8) -> BKAWL_W<'_, AWLTRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        BKAWL_W::new(self, n)
    }
    ///Bit 0 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl0(&mut self) -> BKAWL_W<'_, AWLTRrs> {
        BKAWL_W::new(self, 0)
    }
    ///Bit 1 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl1(&mut self) -> BKAWL_W<'_, AWLTRrs> {
        BKAWL_W::new(self, 1)
    }
    ///Bit 2 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl2(&mut self) -> BKAWL_W<'_, AWLTRrs> {
        BKAWL_W::new(self, 2)
    }
    ///Bit 3 - Break signal assignment to analog watchdog low threshold event
    #[inline(always)]
    pub fn bkawl3(&mut self) -> BKAWL_W<'_, AWLTRrs> {
        BKAWL_W::new(self, 3)
    }
    ///Bits 8:31 - Analog watchdog low threshold
    #[inline(always)]
    pub fn awlt(&mut self) -> AWLT_W<'_, AWLTRrs> {
        AWLT_W::new(self, 8)
    }
}
/**analog watchdog low threshold register

You can [`read`](crate::Reg::read) this register and get [`awltr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awltr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AWLTRrs;
impl crate::RegisterSpec for AWLTRrs {
    type Ux = u32;
}
///`read()` method returns [`awltr::R`](R) reader structure
impl crate::Readable for AWLTRrs {}
///`write(|w| ..)` method takes [`awltr::W`](W) writer structure
impl crate::Writable for AWLTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AWLTR to value 0
impl crate::Resettable for AWLTRrs {}
