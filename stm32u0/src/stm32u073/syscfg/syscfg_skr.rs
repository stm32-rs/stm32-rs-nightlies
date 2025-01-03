///Register `SYSCFG_SKR` writer
pub type W = crate::W<SYSCFG_SKRrs>;
/**Field `KEY` writer - SRAM2 write protection key for software erase The following steps are required to unlock the write protection of the SRAM2ER bit in the SYSCFG_CFGR2 register: Write 0xCA into KEY\[7:0\]
Write 0x53 into KEY\[7:0\]
Writing a wrong key reactivates the write protection.*/
pub type KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<SYSCFG_SKRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    /**Bits 0:7 - SRAM2 write protection key for software erase The following steps are required to unlock the write protection of the SRAM2ER bit in the SYSCFG_CFGR2 register: Write 0xCA into KEY\[7:0\]
    Write 0x53 into KEY\[7:0\]
    Writing a wrong key reactivates the write protection.*/
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<SYSCFG_SKRrs> {
        KEY_W::new(self, 0)
    }
}
/**SYSCFG SRAM2 key register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg_skr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#SYSCFG:SYSCFG_SKR)*/
pub struct SYSCFG_SKRrs;
impl crate::RegisterSpec for SYSCFG_SKRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`syscfg_skr::W`](W) writer structure
impl crate::Writable for SYSCFG_SKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYSCFG_SKR to value 0
impl crate::Resettable for SYSCFG_SKRrs {
    const RESET_VALUE: u32 = 0;
}
