///Register `M1CR` reader
pub type R = crate::R<M1CRrs>;
///Register `M1CR` writer
pub type W = crate::W<M1CRrs>;
///Field `SRAMER` reader - SRAM1 erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_M1ERKEYR register. Setting this bit starts the SRAM1 erase. This bit is automatically cleared by hardware at the end of the erase operation.
pub type SRAMER_R = crate::BitReader;
///Field `SRAMER` writer - SRAM1 erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_M1ERKEYR register. Setting this bit starts the SRAM1 erase. This bit is automatically cleared by hardware at the end of the erase operation.
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WSC` reader - SRAM1 wait state configuration This field is used to program the number of wait states inserted on the AHB when reading the SRAM1, depending on its access time. ... Note: Before entering Stop 1 mode software must set SRAM1 wait states to at least 1.
pub type WSC_R = crate::FieldReader;
///Field `WSC` writer - SRAM1 wait state configuration This field is used to program the number of wait states inserted on the AHB when reading the SRAM1, depending on its access time. ... Note: Before entering Stop 1 mode software must set SRAM1 wait states to at least 1.
pub type WSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 8 - SRAM1 erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_M1ERKEYR register. Setting this bit starts the SRAM1 erase. This bit is automatically cleared by hardware at the end of the erase operation.
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:18 - SRAM1 wait state configuration This field is used to program the number of wait states inserted on the AHB when reading the SRAM1, depending on its access time. ... Note: Before entering Stop 1 mode software must set SRAM1 wait states to at least 1.
    #[inline(always)]
    pub fn wsc(&self) -> WSC_R {
        WSC_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M1CR")
            .field("sramer", &self.sramer())
            .field("wsc", &self.wsc())
            .finish()
    }
}
impl W {
    ///Bit 8 - SRAM1 erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the RAMCFG_M1ERKEYR register. Setting this bit starts the SRAM1 erase. This bit is automatically cleared by hardware at the end of the erase operation.
    #[inline(always)]
    pub fn sramer(&mut self) -> SRAMER_W<'_, M1CRrs> {
        SRAMER_W::new(self, 8)
    }
    ///Bits 16:18 - SRAM1 wait state configuration This field is used to program the number of wait states inserted on the AHB when reading the SRAM1, depending on its access time. ... Note: Before entering Stop 1 mode software must set SRAM1 wait states to at least 1.
    #[inline(always)]
    pub fn wsc(&mut self) -> WSC_W<'_, M1CRrs> {
        WSC_W::new(self, 16)
    }
}
/**RAMCFG SRAM1 control register

You can [`read`](crate::Reg::read) this register and get [`m1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA52.html#RAMCFG:M1CR)*/
pub struct M1CRrs;
impl crate::RegisterSpec for M1CRrs {
    type Ux = u32;
}
///`read()` method returns [`m1cr::R`](R) reader structure
impl crate::Readable for M1CRrs {}
///`write(|w| ..)` method takes [`m1cr::W`](W) writer structure
impl crate::Writable for M1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M1CR to value 0x0001_0000
impl crate::Resettable for M1CRrs {
    const RESET_VALUE: u32 = 0x0001_0000;
}
