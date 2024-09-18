///Register `BRR` reader
pub type R = crate::R<BRRrs>;
///Register `BRR` writer
pub type W = crate::W<BRRrs>;
/**Port Reset bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR3W {
    ///0: No action on the corresponding ODx bit
    NoAction = 0,
    ///1: Reset the ODx bit
    Reset = 1,
}
impl From<BR3W> for bool {
    #[inline(always)]
    fn from(variant: BR3W) -> Self {
        variant as u8 != 0
    }
}
///Field `BR3` reader - Port Reset bit
pub type BR3_R = crate::BitReader<BR3W>;
impl BR3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BR3W {
        match self.bits {
            false => BR3W::NoAction,
            true => BR3W::Reset,
        }
    }
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == BR3W::NoAction
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BR3W::Reset
    }
}
///Field `BR3` writer - Port Reset bit
pub type BR3_W<'a, REG> = crate::BitWriter<'a, REG, BR3W>;
impl<'a, REG> BR3_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut crate::W<REG> {
        self.variant(BR3W::NoAction)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(BR3W::Reset)
    }
}
impl R {
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    pub fn br3(&self) -> BR3_R {
        BR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BRR").field("br3", &self.br3()).finish()
    }
}
impl W {
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<BRRrs> {
        BR3_W::new(self, 3)
    }
}
/**GPIO port bit reset register

You can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#GPIOH:BRR)*/
pub struct BRRrs;
impl crate::RegisterSpec for BRRrs {
    type Ux = u32;
}
///`read()` method returns [`brr::R`](R) reader structure
impl crate::Readable for BRRrs {}
///`write(|w| ..)` method takes [`brr::W`](W) writer structure
impl crate::Writable for BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRRrs {
    const RESET_VALUE: u32 = 0;
}
