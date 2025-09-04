///Register `CFR` writer
pub type W = crate::W<CFRrs>;
/**Synchronization Clear Overrun Flag %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSOF0W {
    ///1: Clear synchronization flag
    Clear = 1,
}
impl From<CSOF0W> for bool {
    #[inline(always)]
    fn from(variant: CSOF0W) -> Self {
        variant as u8 != 0
    }
}
///Field `CSOF(0-2)` writer - Synchronization Clear Overrun Flag %s
pub type CSOF_W<'a, REG> = crate::BitWriter1C<'a, REG, CSOF0W>;
impl<'a, REG> CSOF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear synchronization flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CSOF0W::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<CFRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Synchronization Clear Overrun Flag (0-2)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CSOF0` field.</div>
    #[inline(always)]
    pub fn csof(&mut self, n: u8) -> CSOF_W<CFRrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        CSOF_W::new(self, n)
    }
    ///Bit 0 - Synchronization Clear Overrun Flag 0
    #[inline(always)]
    pub fn csof0(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 0)
    }
    ///Bit 1 - Synchronization Clear Overrun Flag 1
    #[inline(always)]
    pub fn csof1(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 1)
    }
    ///Bit 2 - Synchronization Clear Overrun Flag 2
    #[inline(always)]
    pub fn csof2(&mut self) -> CSOF_W<CFRrs> {
        CSOF_W::new(self, 2)
    }
}
/**DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#DMAMUX:CFR)*/
pub struct CFRrs;
impl crate::RegisterSpec for CFRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`cfr::W`](W) writer structure
impl crate::Writable for CFRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
///`reset()` method sets CFR to value 0
impl crate::Resettable for CFRrs {}
