///Register `CSCMDR` reader
pub type R = crate::R<CSCMDRrs>;
///Register `CSCMDR` writer
pub type W = crate::W<CSCMDRrs>;
///Field `REQUEST` reader - Request for system clock switching Cleared by hardware when system clock frequency switch is done
pub type REQUEST_R = crate::BitReader;
///Field `REQUEST` writer - Request for system clock switching Cleared by hardware when system clock frequency switch is done
pub type REQUEST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKSYSDIV_REQ` reader - system clock dividing factor from HSI_64M requested Note: behavior depends on BLEEN in APB2ENR register
pub type CLKSYSDIV_REQ_R = crate::FieldReader;
///Field `CLKSYSDIV_REQ` writer - system clock dividing factor from HSI_64M requested Note: behavior depends on BLEEN in APB2ENR register
pub type CLKSYSDIV_REQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `STATUS` reader - Status of clock switch sequence
pub type STATUS_R = crate::FieldReader;
///Field `EOFSEQ_IE` reader - End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch.
pub type EOFSEQ_IE_R = crate::BitReader;
///Field `EOFSEQ_IE` writer - End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch.
pub type EOFSEQ_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOFSEQ_IRQ` reader - End of Sequence flag Set by hardware when clock system swtich is ended
pub type EOFSEQ_IRQ_R = crate::BitReader;
///Field `EOFSEQ_IRQ` writer - End of Sequence flag Set by hardware when clock system swtich is ended
pub type EOFSEQ_IRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Request for system clock switching Cleared by hardware when system clock frequency switch is done
    #[inline(always)]
    pub fn request(&self) -> REQUEST_R {
        REQUEST_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:3 - system clock dividing factor from HSI_64M requested Note: behavior depends on BLEEN in APB2ENR register
    #[inline(always)]
    pub fn clksysdiv_req(&self) -> CLKSYSDIV_REQ_R {
        CLKSYSDIV_REQ_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bits 4:5 - Status of clock switch sequence
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch.
    #[inline(always)]
    pub fn eofseq_ie(&self) -> EOFSEQ_IE_R {
        EOFSEQ_IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - End of Sequence flag Set by hardware when clock system swtich is ended
    #[inline(always)]
    pub fn eofseq_irq(&self) -> EOFSEQ_IRQ_R {
        EOFSEQ_IRQ_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSCMDR")
            .field("request", &self.request())
            .field("clksysdiv_req", &self.clksysdiv_req())
            .field("status", &self.status())
            .field("eofseq_ie", &self.eofseq_ie())
            .field("eofseq_irq", &self.eofseq_irq())
            .finish()
    }
}
impl W {
    ///Bit 0 - Request for system clock switching Cleared by hardware when system clock frequency switch is done
    #[inline(always)]
    pub fn request(&mut self) -> REQUEST_W<'_, CSCMDRrs> {
        REQUEST_W::new(self, 0)
    }
    ///Bits 1:3 - system clock dividing factor from HSI_64M requested Note: behavior depends on BLEEN in APB2ENR register
    #[inline(always)]
    pub fn clksysdiv_req(&mut self) -> CLKSYSDIV_REQ_W<'_, CSCMDRrs> {
        CLKSYSDIV_REQ_W::new(self, 1)
    }
    ///Bit 6 - End of sequence Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the clock system switch.
    #[inline(always)]
    pub fn eofseq_ie(&mut self) -> EOFSEQ_IE_W<'_, CSCMDRrs> {
        EOFSEQ_IE_W::new(self, 6)
    }
    ///Bit 7 - End of Sequence flag Set by hardware when clock system swtich is ended
    #[inline(always)]
    pub fn eofseq_irq(&mut self) -> EOFSEQ_IRQ_W<'_, CSCMDRrs> {
        EOFSEQ_IRQ_W::new(self, 7)
    }
}
/**CSCMDR register

You can [`read`](crate::Reg::read) this register and get [`cscmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RCC:CSCMDR)*/
pub struct CSCMDRrs;
impl crate::RegisterSpec for CSCMDRrs {
    type Ux = u32;
}
///`read()` method returns [`cscmdr::R`](R) reader structure
impl crate::Readable for CSCMDRrs {}
///`write(|w| ..)` method takes [`cscmdr::W`](W) writer structure
impl crate::Writable for CSCMDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSCMDR to value 0x80
impl crate::Resettable for CSCMDRrs {
    const RESET_VALUE: u32 = 0x80;
}
