///Register `TS0HILORESETR` writer
pub type W = crate::W<TS0HILORESETRrs>;
///Field `SMPL_LO_SET` writer - Sample Low Set
pub type SMPL_LO_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMPL_HI_CLR` writer - Sample high clear 0
pub type SMPL_HI_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TS0HILORESETRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Sample Low Set
    #[inline(always)]
    pub fn smpl_lo_set(&mut self) -> SMPL_LO_SET_W<'_, TS0HILORESETRrs> {
        SMPL_LO_SET_W::new(self, 0)
    }
    ///Bit 1 - Sample high clear 0
    #[inline(always)]
    pub fn smpl_hi_clr(&mut self) -> SMPL_HI_CLR_W<'_, TS0HILORESETRrs> {
        SMPL_HI_CLR_W::new(self, 1)
    }
}
/**DTS TS0 high/low reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ts0hiloresetr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DTS:TS0HILORESETR)*/
pub struct TS0HILORESETRrs;
impl crate::RegisterSpec for TS0HILORESETRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ts0hiloresetr::W`](W) writer structure
impl crate::Writable for TS0HILORESETRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TS0HILORESETR to value 0
impl crate::Resettable for TS0HILORESETRrs {}
