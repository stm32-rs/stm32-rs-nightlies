///Register `OTFDEC_R1NONCER0` reader
pub type R = crate::R<OTFDEC_R1NONCER0rs>;
///Register `OTFDEC_R1NONCER0` writer
pub type W = crate::W<OTFDEC_R1NONCER0rs>;
/**Field `REGx_NONCE` reader - Region nonce, bits \[31:0\]
This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded.*/
pub type REGX_NONCE_R = crate::FieldReader<u32>;
/**Field `REGx_NONCE` writer - Region nonce, bits \[31:0\]
This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded.*/
pub type REGX_NONCE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    /**Bits 0:31 - Region nonce, bits \[31:0\]
    This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded.*/
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTFDEC_R1NONCER0")
            .field("regx_nonce", &self.regx_nonce())
            .finish()
    }
}
impl W {
    /**Bits 0:31 - Region nonce, bits \[31:0\]
    This register must be written before the region corresponding REG_EN bit in the RxCFGR register is set. Writing this register while the region CONFIGLOCK bit in the RxCFGR register is set will be discarded.*/
    #[inline(always)]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W<OTFDEC_R1NONCER0rs> {
        REGX_NONCE_W::new(self, 0)
    }
}
/**OTFDEC region 1 nonce register 0

You can [`read`](crate::Reg::read) this register and get [`otfdec_r1noncer0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_r1noncer0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#OTFDEC1:OTFDEC_R1NONCER0)*/
pub struct OTFDEC_R1NONCER0rs;
impl crate::RegisterSpec for OTFDEC_R1NONCER0rs {
    type Ux = u32;
}
///`read()` method returns [`otfdec_r1noncer0::R`](R) reader structure
impl crate::Readable for OTFDEC_R1NONCER0rs {}
///`write(|w| ..)` method takes [`otfdec_r1noncer0::W`](W) writer structure
impl crate::Writable for OTFDEC_R1NONCER0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTFDEC_R1NONCER0 to value 0
impl crate::Resettable for OTFDEC_R1NONCER0rs {
    const RESET_VALUE: u32 = 0;
}
