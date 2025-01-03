///Register `USB_DADDR` reader
pub type R = crate::R<USB_DADDRrs>;
///Register `USB_DADDR` writer
pub type W = crate::W<USB_DADDRrs>;
///Field `ADD` reader - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel address (EA) field in the associated USB_CHEPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction
pub type ADD_R = crate::FieldReader;
///Field `ADD` writer - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel address (EA) field in the associated USB_CHEPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction
pub type ADD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
/**Field `EF` reader - Enable function This bit is set by the software to enable the USB Device. The address of this device is contained in the following ADD\[6:0\]
bits. If this bit is at 0 no transactions are handled, irrespective of the settings of USB_CHEPnR registers.*/
pub type EF_R = crate::BitReader;
/**Field `EF` writer - Enable function This bit is set by the software to enable the USB Device. The address of this device is contained in the following ADD\[6:0\]
bits. If this bit is at 0 no transactions are handled, irrespective of the settings of USB_CHEPnR registers.*/
pub type EF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel address (EA) field in the associated USB_CHEPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
    /**Bit 7 - Enable function This bit is set by the software to enable the USB Device. The address of this device is contained in the following ADD\[6:0\]
    bits. If this bit is at 0 no transactions are handled, irrespective of the settings of USB_CHEPnR registers.*/
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_DADDR")
            .field("add", &self.add())
            .field("ef", &self.ef())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Device address Device mode These bits contain the USB function address assigned by the host PC during the enumeration process. Both this field and the endpoint/channel address (EA) field in the associated USB_CHEPnR register must match with the information contained in a USB token in order to handle a transaction to the required endpoint. Host mode These bits contain the address transmitted with the LPM transaction
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<USB_DADDRrs> {
        ADD_W::new(self, 0)
    }
    /**Bit 7 - Enable function This bit is set by the software to enable the USB Device. The address of this device is contained in the following ADD\[6:0\]
    bits. If this bit is at 0 no transactions are handled, irrespective of the settings of USB_CHEPnR registers.*/
    #[inline(always)]
    pub fn ef(&mut self) -> EF_W<USB_DADDRrs> {
        EF_W::new(self, 7)
    }
}
/**USB Device address

You can [`read`](crate::Reg::read) this register and get [`usb_daddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_daddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#USB:USB_DADDR)*/
pub struct USB_DADDRrs;
impl crate::RegisterSpec for USB_DADDRrs {
    type Ux = u32;
}
///`read()` method returns [`usb_daddr::R`](R) reader structure
impl crate::Readable for USB_DADDRrs {}
///`write(|w| ..)` method takes [`usb_daddr::W`](W) writer structure
impl crate::Writable for USB_DADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets USB_DADDR to value 0
impl crate::Resettable for USB_DADDRrs {
    const RESET_VALUE: u32 = 0;
}
