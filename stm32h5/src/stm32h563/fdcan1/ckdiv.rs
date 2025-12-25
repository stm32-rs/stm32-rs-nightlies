///Register `CKDIV` reader
pub type R = crate::R<CKDIVrs>;
///Register `CKDIV` writer
pub type W = crate::W<CKDIVrs>;
///Field `PDIV` reader - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type PDIV_R = crate::FieldReader;
///Field `PDIV` writer - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
pub type PDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn pdiv(&self) -> PDIV_R {
        PDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CKDIV").field("pdiv", &self.pdiv()).finish()
    }
}
impl W {
    ///Bits 0:3 - input clock divider The APB clock could be divided prior to be used by the CAN sub system. The rate must be computed using the divider output clock. These are protected write (P) bits, which means that write access by the bits is possible only when the bit 1 \[CCE\] and bit 0 \[INIT\] of CCCR register are set to 1.
    #[inline(always)]
    pub fn pdiv(&mut self) -> PDIV_W<'_, CKDIVrs> {
        PDIV_W::new(self, 0)
    }
}
/**FDCAN CFG clock divider register

You can [`read`](crate::Reg::read) this register and get [`ckdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H563.html#FDCAN1:CKDIV)*/
pub struct CKDIVrs;
impl crate::RegisterSpec for CKDIVrs {
    type Ux = u32;
}
///`read()` method returns [`ckdiv::R`](R) reader structure
impl crate::Readable for CKDIVrs {}
///`write(|w| ..)` method takes [`ckdiv::W`](W) writer structure
impl crate::Writable for CKDIVrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CKDIV to value 0
impl crate::Resettable for CKDIVrs {}
