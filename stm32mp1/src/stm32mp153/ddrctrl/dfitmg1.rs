///Register `DFITMG1` reader
pub type R = crate::R<DFITMG1rs>;
///Register `DFITMG1` writer
pub type W = crate::W<DFITMG1rs>;
///Field `DFI_T_DRAM_CLK_ENABLE` reader - DFI_T_DRAM_CLK_ENABLE
pub type DFI_T_DRAM_CLK_ENABLE_R = crate::FieldReader;
///Field `DFI_T_DRAM_CLK_ENABLE` writer - DFI_T_DRAM_CLK_ENABLE
pub type DFI_T_DRAM_CLK_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DFI_T_DRAM_CLK_DISABLE` reader - DFI_T_DRAM_CLK_DISABLE
pub type DFI_T_DRAM_CLK_DISABLE_R = crate::FieldReader;
///Field `DFI_T_DRAM_CLK_DISABLE` writer - DFI_T_DRAM_CLK_DISABLE
pub type DFI_T_DRAM_CLK_DISABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DFI_T_WRDATA_DELAY` reader - DFI_T_WRDATA_DELAY
pub type DFI_T_WRDATA_DELAY_R = crate::FieldReader;
///Field `DFI_T_WRDATA_DELAY` writer - DFI_T_WRDATA_DELAY
pub type DFI_T_WRDATA_DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - DFI_T_DRAM_CLK_ENABLE
    #[inline(always)]
    pub fn dfi_t_dram_clk_enable(&self) -> DFI_T_DRAM_CLK_ENABLE_R {
        DFI_T_DRAM_CLK_ENABLE_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - DFI_T_DRAM_CLK_DISABLE
    #[inline(always)]
    pub fn dfi_t_dram_clk_disable(&self) -> DFI_T_DRAM_CLK_DISABLE_R {
        DFI_T_DRAM_CLK_DISABLE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:20 - DFI_T_WRDATA_DELAY
    #[inline(always)]
    pub fn dfi_t_wrdata_delay(&self) -> DFI_T_WRDATA_DELAY_R {
        DFI_T_WRDATA_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DFITMG1")
            .field("dfi_t_dram_clk_enable", &self.dfi_t_dram_clk_enable())
            .field("dfi_t_dram_clk_disable", &self.dfi_t_dram_clk_disable())
            .field("dfi_t_wrdata_delay", &self.dfi_t_wrdata_delay())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - DFI_T_DRAM_CLK_ENABLE
    #[inline(always)]
    pub fn dfi_t_dram_clk_enable(&mut self) -> DFI_T_DRAM_CLK_ENABLE_W<'_, DFITMG1rs> {
        DFI_T_DRAM_CLK_ENABLE_W::new(self, 0)
    }
    ///Bits 8:12 - DFI_T_DRAM_CLK_DISABLE
    #[inline(always)]
    pub fn dfi_t_dram_clk_disable(&mut self) -> DFI_T_DRAM_CLK_DISABLE_W<'_, DFITMG1rs> {
        DFI_T_DRAM_CLK_DISABLE_W::new(self, 8)
    }
    ///Bits 16:20 - DFI_T_WRDATA_DELAY
    #[inline(always)]
    pub fn dfi_t_wrdata_delay(&mut self) -> DFI_T_WRDATA_DELAY_W<'_, DFITMG1rs> {
        DFI_T_WRDATA_DELAY_W::new(self, 16)
    }
}
/**DDRCTRL DFI timing register 1

You can [`read`](crate::Reg::read) this register and get [`dfitmg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfitmg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DDRCTRL:DFITMG1)*/
pub struct DFITMG1rs;
impl crate::RegisterSpec for DFITMG1rs {
    type Ux = u32;
}
///`read()` method returns [`dfitmg1::R`](R) reader structure
impl crate::Readable for DFITMG1rs {}
///`write(|w| ..)` method takes [`dfitmg1::W`](W) writer structure
impl crate::Writable for DFITMG1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DFITMG1 to value 0x0404
impl crate::Resettable for DFITMG1rs {
    const RESET_VALUE: u32 = 0x0404;
}
